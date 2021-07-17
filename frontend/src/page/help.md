# how to:
#### To pull and push images you first need to sign in using Gamma with:

```
docker login docker.chalmers.it
```
#### To pull an image use:
```
docker pull docker.chalmers.it/[repo_path]:[tag]
```
or just
```
docker pull docker.chalmers.it/[repo_path]
```
to pull `:latest`

#### To push an image:
##### *(only digIT and didIT have push access)*

you first have to create a tag that starts with `docker.chalmers.it`

if you have an existing image it can be re-tagged by using
```
docker tag [repo] docker.chalmers.it/[repo]:[tag]
```
or you can tag an image when you build it by using
```
docker build -t docker.chalmers.it/[repo]:[tag] .
```
if a tag is omitted the default tag is `:latest`

then you can push the image to `docker.chalmers.it` with
```
docker push docker.chalmers.it/[repo]:[tag]
```
