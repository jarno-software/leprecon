FROM openresty/openresty:1.25.3.1-2-alpine-fat

# Remove default html
RUN rm -rf /usr/local/openresty/nginx/html

# Copy files
COPY www/ /usr/local/openresty/nginx/html
COPY nginx/nginx.conf /usr/local/openresty/nginx/conf/nginx.conf
COPY nginx/nginx.conf.template /etc/nginx/conf.d/server.conf.template
COPY nginx/lua/ /etc/nginx/
COPY nginx/mime.types /usr/local/openresty/nginx/conf/mime.types

# Entrypoint script
COPY entrypoint.sh /etc/nginx/entrypoint.sh

# Install dependencies for jwt authorisation
RUN apk update && apk upgrade && apk add pkgconfig openssl openssl-dev certbot
RUN opm get SkyLothar/lua-resty-jwt
RUN luarocks install openssl
RUN luarocks install lua-resty-rsa
RUN luarocks install lua-resty-openssl
RUN luarocks install lua-resty-auto-ssl
RUN mkdir /etc/resty-auto-ssl
RUN chown root /etc/resty-auto-ssl
RUN openssl req -new -newkey rsa:2048 -days 3650 -nodes -x509 \
      -subj '/CN=sni-support-required-for-valid-ssl' \
      -keyout /etc/ssl/resty-auto-ssl-fallback.key \
      -out /etc/ssl/resty-auto-ssl-fallback.crt
ENTRYPOINT ["/etc/nginx/entrypoint.sh"]

# Start NGINX
CMD ["nginx", "-g", "daemon off;"]
