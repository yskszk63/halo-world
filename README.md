# halo-world

A docker image running a simple HTTP server.

```bash
$ docker run -d -ePORT=3000 -p3000:3000 yskszk63/halo-world
b2a7949a5b2ca3d961a10d07cf17992907637204f3d389932811c79ff5e7a129
$ curl localhost:3000 -i
HTTP/1.1 200 OK
content-length: 4
date: Thu, 24 Aug 2023 15:17:40 GMT

😇$ 
```

## Purpose

- For AWS App Runner initial deployment image.
    - Deploying AWS App Runner via CloudFormation requires a working container.

# LICENSE

[MIT](LICENSE)
