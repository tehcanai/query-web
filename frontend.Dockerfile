FROM node:22-alpine
WORKDIR /frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend ./
RUN npm run build
CMD ["nginx", "-g", "daemon off;"]
