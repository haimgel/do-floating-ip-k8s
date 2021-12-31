# Anchor IP annotator for Digital Ocean

## What does it do, and why?

* **What**: It adds an annotation to the _pod_ this code is running in, specifying the _anchor IP_
of the node.
* **Why**: Digital Ocean's floating IPs are routed to the Anchor IPs, and Anchor IPs of each
  host are not discoverable. 

## How?

Add this container as an `initContainer`, make sure pod's name and namespace are exposed
as environment variables:

```yaml
initContainers:
- name: anchor-ip-annotator
  image:
  env:
    - name: DIGITAL_OCEAN_METADATA_URL
      value: http://169.254.169.254/metadata/v1.json
    - name: POD_NAME
      valueFrom:
        fieldRef:
          fieldPath: metadata.name
    - name: POD_NAMESPACE
      valueFrom:
        fieldRef:
          fieldPath: metadata.namespace
```
