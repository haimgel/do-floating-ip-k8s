# Floating IPs controller for DigitalOcean K8S platform

Use this Floating IP controller to manage floating IPs for your DigitalOcean K8S cluster.

**This controller will make sure the floating IP is attached to the node that is running the pods that have the
annotation with that IP** ([more info](https://haim.dev/posts/2021-12-30-floating-ip-on-digital-ocean-k8s/)).

# How?

1. Annotate and label your controller pods:
```yaml
  annotations:
    k8s.haim.dev/floating-ip: "104.248.100.100"
  labels:
    k8s.haim.dev/floating-ip: "true"
```

2. Add anchor IPs to your service spec:
```yaml
apiVersion: v1
kind: Service
metadata:
  name: traefik
spec:
  type: ClusterIP
  externalIPs:
    - 10.20.0.1
    - 10.20.0.2
    - 10.20.0.3
    - 10.20.0.4
    - 10.20.0.5
    - 10.20.0.6
    - 10.20.0.7
    - 10.20.0.8
  ... 
```

Configure RBAC, Digital Ocean token, start the controller as a Deployment in your cluster. 
See [some examples](./kubernetes) of how to do it, or read my [blog post](https://haim.dev/posts/2021-12-30-floating-ip-on-digital-ocean-k8s/) for more details.

### Challenge: Anchor IPs

DigitalOcean's nodes do not have the actual floating IP assigned to them, they have "anchor IPs", which receive the 
traffic destined to the Floating IP. However, the Anchor IPs are not exposed on the platform in any way, and we need to 
assign this anchor IP to the service (in the `ExternalIPs` list) in order to receive the traffic. 

Right now I just list the whole lot of them (see above). If your cluster is small, it's not a problem since the IPs
are assigned sequentially, and are eventually reused.

Eventually, I'd want to assign only the needed anchor IPs to the service. I've started with the "run once" 
[anchor-ip-annotator](./src/bin/anchor-ip-annotator.rs) DaemonSet to annotate all the nodes with the anchor IPs, 
and then the idea is to apply these to the Service. But this code is not written yet.

### Challenge: Firewall

DigitalOcean's K8S platform automatically blocks all traffic to the nodes using the firewall. Since we DO want to let
the traffic in:
 * Cannot change the _default_ firweall managed by K8S itself, since any changes will be overwritten.
 * Can create a _new_ firewall and target the same nodes using tags, adding the necessary exceptions there.
 * **TODO**: do it automatically in the controller.

