package consumers

import (
	"encoding/json"
	"fmt"
	"github.com/mailgun/mailgun-go"
	"github.com/wagslane/go-rabbitmq"
	"log"
	"os"
)

var mg = mailgun.NewMailgun("mg.feal.io", os.Getenv("MAILGUN_API"))

type verifyEmailMessage struct {
	Code  string `json:"code"`
	Email string `json:"email"`
}

func VerifyEmailHandler(d rabbitmq.Delivery) rabbitmq.Action {
	log.Printf("consumed: %v", string(d.Body))

	var message verifyEmailMessage
	err := json.Unmarshal(d.Body, &message)
	if err != nil {
		log.Printf("discarded: %s", err.Error())
		return rabbitmq.NackDiscard
	}

	_, _, err = mg.Send(mg.NewMessage(
		"My shop <mailgun@mg.feal.io>",
		"Your verification code",
		fmt.Sprintf("Here is the verification code to create your account: %s", message.Code),
		message.Email,
	))
	if err != nil {
		log.Printf("requeued: %s", err.Error())
		return rabbitmq.NackRequeue
	}

	return rabbitmq.Ack
}
