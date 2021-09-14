# Istio Filters

## Minikube setup

```sh
minikube start --nodes=2 --memory=4g --network-plugin=cni --cni=calico --kubernetes-version=1.21.4
```

## Installing istio

Make sure the minikube context is selected

```sh
kubectl config get-contexts
```

Install istio:

```sh
istioctl install --set profile=demo -y
```

Check if the ingress and egress gateways start up correctly. I often seem to have issues in a nwe minikube cluster with dns. If you see cert errors this is most likely due to an error earlier where the gateways can't resolve the istiod DNS.

## Installing addons

```sh
kubectl apply -f services/prometheus.yaml
kubectl apply -f services/grafana.yaml
kubectl apply -f services/kiali.yaml
```

Expose services:

```sh
kubectl apply -f services/kiali-gateway.yaml
```

## Expose your loadbalancer with minikube

At the moment services withing the cluster aren't routable because the ingress gateway doesn't have an IP yet. The external IP is still pending

```sh
➜  ~ kubectl get -n istio-system services istio-ingressgateway
NAME                   TYPE           CLUSTER-IP       EXTERNAL-IP   PORT(S)                                                                      AGE
istio-ingressgateway   LoadBalancer   10.105.117.131   <pending>     15021:30767/TCP,80:31368/TCP,443:30987/TCP,31400:31133/TCP,15443:31623/TCP   15m
```

To assign an IP to the LB run the following minikube command

```sh
minikube tunnel --cleanup
```

Now the LB should get an IP

```sh
➜  ~ kubectl get -n istio-system services istio-ingressgateway
NAME                   TYPE           CLUSTER-IP       EXTERNAL-IP      PORT(S)                                                                      AGE
istio-ingressgateway   LoadBalancer   10.105.117.131   10.105.117.131   15021:30767/TCP,80:31368/TCP,443:30987/TCP,31400:31133/TCP,15443:31623/TCP   16m
```

Curl should now be able to hit the LB:

```sh
➜  ~ curl -i 10.105.117.131
HTTP/1.1 404 Not Found
date: Mon, 13 Sep 2021 07:40:44 GMT
server: istio-envoy
content-length: 0

```

Lets quickly add kiali to the hosts file so it's routable from a browser

`sudo vim /etc/hosts`

And add the external ip with `kiali.mux.local`

## Setting up the playground

Create a namespace with sidecar injection enabled

```sh
kubectl create namespace filters
kubectl label namespace filters istio-injection=enabled
```

Install httpbin

```sh
kubectl apply -n filters -f apps/httpbin.yaml
kubectl apply -n filters -f apps/sleep.yaml
```

Annotate istiod

```yaml
metadata:
  annotations:
    sidecar.istio.io/userVolume: '[{"name":"http-filter","configMap":{"name":"http-filter"}}]'
    sidecar.istio.io/userVolumeMount: '[{"mountPath":"/var/local/wasm","name":"http-filter"}]'
```

Set log level for an istio proxy

```sh
istioctl pc -n filters log httpbin-7465f67698-fc6lb --level wasm:info
```

Deploy wasm module

```sh
kubectl delete -n filters configmap jwt-claims-header-filter
kubectl create -n filters configmap jwt-claims-header-filter  --from-file=jwt_claims_header_filter.wasm=jwt_claims_header_module.wasm --from-file=jwt_from_cookie_filter.wasm=jwt_from_cookie_module.wasm
kubectl rollout restart -n filters deployment httpbin
sleep 5
istioctl pc -n filters log  $(kubectl get -n filters pods -l app=httpbin -o jsonpath={.items..metadata.name}) --level wasm:info
```
