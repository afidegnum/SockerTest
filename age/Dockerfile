FROM postgres:16

RUN apt-get update \
    && apt-get install -y --no-install-recommends --no-install-suggests \
    bison \
    build-essential \
    flex \
    ncat \
    netcat-openbsd \
    postgresql-server-dev-16 \
    locales

ENV LANG=en_US.UTF-8
ENV LC_COLLATE=en_US.UTF-8
ENV LC_CTYPE=en_US.UTF-8

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen \
    && locale-gen \
    && update-locale LANG=en_US.UTF-8

COPY . /age

WORKDIR /age

RUN make && make install

# COPY docker/docker-entrypoint-initdb.d/00-create-extension-age.sql /docker-entrypoint-initdb.d/00-create-extension-age.sql

CMD ["postgres", "-c", "shared_preload_libraries=age"]







