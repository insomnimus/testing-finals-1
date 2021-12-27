package main

import (
	"fmt"
	"io"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

type HangReader <-chan bool

func (h *HangReader) Read(_ []byte) (int, error) {
	<-*h
	return 0, io.EOF
}

type DelayedReader struct {
	buf      []byte
	pos      int
	interval time.Duration
}

func (r *DelayedReader) Read(buf []byte) (int, error) {
	for i := 0; i < len(buf); i++ {
		if r.pos >= len(r.buf) {
			return i, io.EOF
		}
		if r.buf[r.pos] == '\n' {
			time.Sleep(r.interval)
		}
		buf[i] = r.buf[r.pos]
		r.pos++
	}
	return len(buf), nil
}

type Command struct {
	name string
	val  string
}

func Msg(s string) Command {
	return Command{name: "msg", val: s}
}

func File(s string) Command {
	return Command{name: "file", val: s}
}

func startListening(cmdPath, wd, key, addr string, done <-chan bool) (string, error) {
	cmd := exec.Command(cmdPath, "-k", key, "listen", addr)
	fakeStdin := HangReader(done)
	cmd.Stdin = &fakeStdin
	cmd.Dir = wd
	out, err := cmd.CombinedOutput()
	return string(out), err
}

func startConnect(cmdPath, wd, key, addr, commands string) (string, error) {
	cmd := exec.Command(cmdPath, "-k", key, "connect", addr)
	fakeStdin := DelayedReader{
		buf:      []byte(commands),
		pos:      0,
		interval: time.Second,
	}
	cmd.Stdin = &fakeStdin
	cmd.Dir = wd
	out, err := cmd.CombinedOutput()
	return string(out), err
}

func createFile(path string) error {
	return os.WriteFile(path, []byte("this is some nonsesnse data\nasldkfjhaweklfjhaksdjfhawekujf\n1234\n\nasdfasd\r\n12\n"), 0o777)
}

func main() {
	log.SetPrefix("")
	log.SetFlags(0)
	if len(os.Args) < 2 {
		log.Fatal("missing argument: command path")
	}
	cmdPath := os.Args[1]
	tmp, err := os.MkdirTemp("", "chat_blackbox_tests_*")
	if err != nil {
		log.Fatalf("failed to create a temporary directory: %s", err)
	}
	defer func() {
		os.RemoveAll(tmp)
	}()

	if err := os.Mkdir(filepath.Join(tmp, "server"), 0o777); err != nil {
		log.Fatalf("failed to create a temporary directory: %s", err)
	}

	// Create dummy files.
	if err := createFile(filepath.Join(tmp, "test1.dat")); err != nil {
		log.Fatalf("failed to create test files: %s", err)
	}
	if err := createFile(filepath.Join(tmp, "test2.dat")); err != nil {
		log.Fatalf("failed to create test files: %s", err)
	}

	serverOut := make(chan string, 1)
	serverDone := make(chan bool, 1)
	go func() {
		out, err := startListening(cmdPath,
			filepath.Join(tmp, "server"),
			"asdf",
			"localhost:5241",
			serverDone)
		if err != nil {
			log.Fatalf("listener returned error: %s", err)
		}
		serverOut <- out
	}()

	time.Sleep(time.Second)
	commands := []Command{
		Msg("Hi."),
		Msg("Sending file."),
		File("test1.dat"),
		Msg("nice file isn't it?"),
		Msg("Ok. I'll send another one."),
		File("test2.dat"),
		Msg("I'm done. Goodbye!"),
		Msg("asdf"),
		Msg("bye"),
	}

	var input strings.Builder
	for _, c := range commands {
		fmt.Fprintf(&input, "%s %s\n", c.name, c.val)
	}
	fmt.Fprintln(&input, "exit")

	conOut, err := startConnect(cmdPath, tmp, "asdf", "localhost:5241", input.String())
	if err != nil {
		log.Fatalf("connect returned error: %s", err)
	}
	fmt.Println(conOut)
	time.Sleep(time.Duration(3 * time.Second))
	serverDone <- true

	received := <-serverOut
	received = strings.TrimSpace(received)
	for i, got := range strings.Split(received, "\n") {
		expected := fmt.Sprintf("%s: %s", commands[i].name, commands[i].val)
		if got != expected {
			log.Fatalf("test failed:\nexpected %s\ngot %s", expected, got)
		}
	}

	fmt.Println("OK. Tests passed.")
}
