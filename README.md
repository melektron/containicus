# containicus

containicus allows managing OCI-like container projects (Docker Compose, ...) on NixOS systems with a focus on feature completeness (supporting even obscure backend features that are seldomly used) and ease-of-devops by combining a fully-declarative NixOS configurations with most of the flexibility offered by the respective container backend.


## Inspiration

This project is greatly inspired by two other systems providing similar functionality:
- [Arion](https://github.com/hercules-ci/arion/): Docker-Compose but with Nix syntax (or as NixOS modules)
- [NixOS oci-containers](https://wiki.nixos.org/wiki/Docker#Docker_Containers_as_systemd_Services): NixOS built-in method of managing docker containers declaratively.


