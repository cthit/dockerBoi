- [ ] hemsida
  - [x] kunna se alla images
  - [x] kunna sa alla tags
  - [ ] ta bort tags (stretch goal)
  - [ ] visa download-länk
  - [ ] CSS BEAUTIFUL CSS
- [ ] grej som snackar med registry-apit
  - [x] kunna hämta alla images
  - [x] kunna hämta alla tags
  - [ ] kunna ta bort images/tags

# Arkitektur Hemsida

backend: rocket
frontend: seed

## API

- `/api/images` - lista alla images & tags

# Registry

https://hub.docker.com/_/registry/
https://docs.docker.com/registry/spec/api/

## Funktioner:

- `get_repositories` GET `/v2/_catalog`
  https://docs.docker.com/registry/spec/api/#listing-repositories
- `get_image_tags` GET `/v2/<name>/tags/list`
  https://docs.docker.com/registry/spec/api/#listing-image-tags
