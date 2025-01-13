## /* * * * * * * * * * * /
##  * SchoolOS Students *
##  * Dockerfile		*
## /* * * * * * * * * * * /

## Build the image with node 18
FROM node:18-alpine3.14 as builder

## Run the following commands in the container
WORKDIR /usr/src/schoolos-students
COPY . .
RUN npm install
RUN npm run build

## Build the image with alpine, run with node (nuxtjs)
FROM node:18-alpine3.14 as runner
WORKDIR /usr/src/schoolos-students
USER root

## Copy the build from the builder image
COPY --from=builder /usr/src/schoolos-students/.output /usr/src/schoolos-students/dist

## Run the binary
CMD ["node", "dist/server/server.mjs"]