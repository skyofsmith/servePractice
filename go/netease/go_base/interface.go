package main

import (
	"fmt"
)

type USB interface {
	Name() string
	Connecter
}

type Connecter interface {
	Connect()
}

type PhoneConnecter struct {
	name string
}

func (pc PhoneConnecter) Name() string {
	return pc.name
}

func (pc PhoneConnecter) Connect() {
	fmt.Println("Connect:", pc.name)
}

func main() {
	//var a USB
	a := PhoneConnecter{"PhoneConnecter"}
	a.Connect()
	DisConnect(a)
}

func DisConnect(usb UBS) {
	if pc, ok := usb.PhoneConnecter; ok {
		fmt.Println("Disconnected.", pc.name)
		return
	}
	fmt.Println("Unknown device.")
}