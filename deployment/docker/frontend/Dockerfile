# BadgerGuard Frontend Dockerfile
FROM node:20-alpine as builder

WORKDIR /build

COPY frontend/admin/package.json frontend/admin/package-lock.json* ./
RUN npm ci

COPY frontend/admin ./
RUN npm run build

# Runtime image
FROM nginx:alpine

COPY --from=builder /build/dist /usr/share/nginx/html
COPY deployment/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

