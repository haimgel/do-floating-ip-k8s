# Floating IPs controller for DigitalOcean K8S platform

## WARNING: This is WIP, not ready for use yet!

## Why?

[DigitalOcean](https://www.digitalocean.com/products/kubernetes/) Kubernetes platform is great. Its Load Balancer
is well-integrated and works great, too, _if you let it terminate your HTTP or HTTPs traffic_. However, if you want
to use something like Traefik to terminate HTTP/HTTPs, you'll face significant issues:

* You won't have the real client's IP address. Once the TCP stream is terminated on the load balancer, only its IP
  address will be visible to you as the source address.
* UDP is not supported. So HTTP/3 is not possible to load-balance, and all other UDP-only services as well.

## How are we going to solve this?

DigitalOcean has Floating IP addresses. This is basically an IP address that can be dynamically assigned to any host.
We are going to bypass the Load Balancer completely, and send the traffic directly to the cluster. A dedicated controller
will monitor the pod(s) in the service, and assign the floating IP to the right node.

### Challenge: Anchor IPs

DigitalOcean's nodes do not have the actual floating IP assigned to them, they have "anchor IPs", which receive the 
traffic destined to the Floating IP. However, the Anchor IPs are not exposed on the platform in any way, and we need to 
assign this anchor IP to the service (in the `ExternalIPs` list) in order to receive the traffic. 

Here's the tiny [do-anchor-ip-annotator](./anchor-ip-annotator) sidecar container that we're going to use to annotate
the pods with the anchor IP address, so the controller can find it later. Add this container as an `initContainer`, 
make sure pod's name and namespace are exposed as environment variables:

```yaml
apiVersion: apps/v1
kind: Deployment
  ....
  spec:
    template:
      spec:
        initContainers:
        - name: do-anchor-ip-annotator
          image: ghcr.io/haimgel/do-anchor-ip-annotator:0.1.0
          env:
            - name: POD_NAME
              valueFrom:
                fieldRef:
                  fieldPath: metadata.name
            - name: POD_NAMESPACE
              valueFrom:
                fieldRef:
                  fieldPath: metadata.namespace
```

### Challenge: Firewall

DigitalOcean's K8S platform automatically blocks all traffic to the nodes using the firewall. Since we DO want to let
the traffic in:
 * Cannot change the _default_ firweall managed by K8S itself, since any changes will be overwritten.
 * Can create a _new_ firewall and target the same nodes using tags, adding the necessary exceptions there.
 * **TODO**: do it automatically in the controller.

