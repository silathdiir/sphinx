// This file is auto-generated by sp1-recursion-compiler.
package verifier

import (
	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/sp1-recursion-gnark/poseidon2"
)

type Circuit struct {
	X frontend.Variable
	Y frontend.Variable
}

func (circuit *Circuit) Define(api frontend.API) error {
	var state [3]frontend.Variable
	p2 := poseidon2.NewPoseidon2Chip(api)
	
	// Variables.
	var var2 frontend.Variable
	var var0 frontend.Variable
	var var1 frontend.Variable
	
	// Operations.
	var0 = frontend.Variable("0")
	var1 = frontend.Variable("1")
	var2 = frontend.Variable("2")
	state = [3]frontend.Variable{var0,var1,var2}
	p2.PermuteMut(&state)
	var0 = state[0]
	var1 = state[1]
	var2 = state[2]
	api.AssertIsEqual(var0, frontend.Variable("5297208644449048816064511434384511824916970985131888684874823260532015509555"))
	api.AssertIsEqual(var1, frontend.Variable("21816030159894113985964609355246484851575571273661473159848781012394295965040"))
	api.AssertIsEqual(var2, frontend.Variable("13940986381491601233448981668101586453321811870310341844570924906201623195336"))
	return nil
}
