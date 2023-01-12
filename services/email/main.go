package main

import (
	"fmt"
	"github.com/JenyaFTW/email/consumers"
	"log"
	"os"
	"os/signal"
	"syscall"

	rabbitmq "github.com/wagslane/go-rabbitmq"
)

func main() {
	connUrl := "amqp://user:user@localhost"
	if mp := os.Getenv("AMQP_URL"); mp != "" {
		connUrl = mp
	}

	conn, err := rabbitmq.NewConn(
		connUrl,
		rabbitmq.WithConnectionOptionsLogging,
	)
	if err != nil {
		log.Fatal(err)
	}
	defer conn.Close()

	verifyEmailConsumer, err := rabbitmq.NewConsumer(
		conn,
		consumers.VerifyEmailHandler,
		"verify_email",
		rabbitmq.WithConsumerOptionsRoutingKey("verify_email"),
		rabbitmq.WithConsumerOptionsExchangeName("emails"),
		rabbitmq.WithConsumerOptionsExchangeDeclare,
	)
	if err != nil {
		log.Fatal(err)
	}
	defer verifyEmailConsumer.Close()

	shipmentUpdateConsumer, err := rabbitmq.NewConsumer(
		conn,
		consumers.ShipmentUpdateMessage,
		"shipment_update",
		rabbitmq.WithConsumerOptionsRoutingKey("shipment_update"),
		rabbitmq.WithConsumerOptionsExchangeName("emails"),
		rabbitmq.WithConsumerOptionsExchangeDeclare,
	)
	if err != nil {
		log.Fatal(err)
	}
	defer shipmentUpdateConsumer.Close()

	sigs := make(chan os.Signal, 1)
	done := make(chan bool, 1)

	signal.Notify(sigs, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		sig := <-sigs
		fmt.Println()
		fmt.Println(sig)
		done <- true
	}()

	fmt.Println("awaiting signal")
	<-done
	fmt.Println("stopping consumer")
}
