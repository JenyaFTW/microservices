package consumers

import (
	"encoding/json"
	"fmt"
	"github.com/wagslane/go-rabbitmq"
	"log"
)

type shipmentUpdateMessage struct {
	Status  string `json:"status"`
	OrderId string `json:"orderId"`
	Email   string `json:"email"`
}

func ShipmentUpdateMessage(d rabbitmq.Delivery) rabbitmq.Action {
	log.Printf("consumed: %v", string(d.Body))

	var message shipmentUpdateMessage
	err := json.Unmarshal(d.Body, &message)
	if err != nil {
		log.Printf("discarded: %s", err.Error())
		return rabbitmq.NackDiscard
	}

	_, _, err = mg.Send(mg.NewMessage(
		"My shop <mailgun@mg.feal.io>",
		fmt.Sprintf("Shipping update for order #%s", message.OrderId),
		fmt.Sprintf("The shipping status for your order has been updated: %s", message.Status),
		message.Email,
	))
	if err != nil {
		log.Printf("requeued: %s", err.Error())
		return rabbitmq.NackRequeue
	}

	return rabbitmq.Ack
}
