FROM maven:3.8.6-openjdk-18

ADD ./pom.xml pom.xml
ADD ./src src/


FROM openjdk:18
COPY ./target/warehouse-srevice-0.0.1-SNAPSHOT.jar warehouse.jar

EXPOSE 3001

CMD ["java", "-jar", "warehouse.jar"]