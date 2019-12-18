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

type TVConnecter struct {
	name string
}

func (tv TVConnecter) Connect() {
	fmt.Println("Connected:", tv.name)
}

func main() {
	//var a USB
	pc := PhoneConnecter{"PhoneConnecter"}
	var a Connecter
	//a.Connect()	// error
	a = Connecter(pc)
	a.Connect()

	pc.name = "pc"
	a.Connect()
	//tv := TVConnecter{"TVConnecter"}
	//var b USB
	//b = USB(tv)
	//b.Connect()
}

func DisConnect(usb USB) {
	switch v := usb.(type) {
	case PhoneConnecter:
		fmt.Println("Disconnected.", v.name)
	default:
		fmt.Println("Unknown device.")
	}
}