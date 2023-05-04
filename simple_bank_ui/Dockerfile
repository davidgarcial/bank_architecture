# Stage 1: Build the Angular app
FROM node:18 as build-stage

WORKDIR /app

COPY package*.json ./

RUN npm install

COPY . .

RUN npm run build --prod

# Stage 2: Set up Nginx to serve the built app
FROM nginx:1.21-alpine

COPY --from=build-stage /app/dist/simple-bank /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
