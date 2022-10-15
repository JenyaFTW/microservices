FROM maven:3.8.6-openjdk-18 AS builder

ADD ./pom.xml pom.xml
ADD ./src src/

RUN mvn clean package

FROM openjdk:18

COPY --from=builder target/warehouse-srevice-0.0.1-SNAPSHOT.jar warehouse.jar

EXPOSE 3001

CMD ["java", "-jar", "warehouse.jar"]