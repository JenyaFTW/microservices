FROM maven:3.8.1-openjdk-17 AS builder

ADD ./pom.xml pom.xml
ADD ./src src/

RUN mvn clean package

FROM openjdk:17

COPY --from=builder target/docker-order-service-0.0.1-SNAPSHOT.jar docker-order-service.jar

EXPOSE 8080:8080

CMD ["java", "-jar", "docker-order-service.jar"]