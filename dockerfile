FROM mysql:5.6.51


ENV MYSQL_ROOT_PASSWORD admin
ENV MYSQL_DATABASE sourcing
ENV MYSQL_USER common_user
ENV MYSQL_PASSWORD password

COPY ./migrations/ /etc/mysql/
VOLUME [ "./db:/var/lib/mysql" ]