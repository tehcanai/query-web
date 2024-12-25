# Stage 1: Build Next.js frontend
FROM node:18-alpine AS frontend-builder
WORKDIR /frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend ./
RUN npm run build

# Stage 2: Build Rocket backend
FROM rust:1.70-alpine AS backend-builder
WORKDIR /backend
COPY backend/Cargo.toml ./
RUN cargo build --release

# Stage 3: Final image -  combines frontend and backend
FROM nginx:alpine

# Copy Next.js build output
COPY --from=frontend-builder /frontend/.next /usr/share/nginx/html/.next
COPY --from=frontend-builder /frontend/public /usr/share/nginx/html/public

# Copy Nginx configuration (replace with your actual config)
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Add backend container and link it via environment variables
ENV BACKEND_URL http://backend:8000

# Start Backend container (Rocket)
RUN apk add --no-cache curl
COPY --from=backend-builder /app/backend/target/release/backend /usr/local/bin/backend
CMD ["/usr/local/bin/backend", "&"]

#Expose nginx port and backend port for testing
EXPOSE 80
EXPOSE 8000
