package builtin

import (
	"github.com/iotaledger/wasp/packages/sctransaction"
	"github.com/iotaledger/wasp/packages/vm"
)

type builtinProcessor map[sctransaction.RequestCode]builtinEntryPoint

type builtinEntryPoint func(ctx vm.Sandbox)

var Processor = builtinProcessor{
	RequestCodeNOP:              nopRequest,
	RequestCodeSetMinimumReward: setMinimumRewardRequest,
	RequestCodeSetDescription:   setDescriptionRequest,
}

func (v *builtinProcessor) GetEntryPoint(code sctransaction.RequestCode) (vm.EntryPoint, bool) {
	if !code.IsReserved() {
		return nil, false
	}
	ep, ok := Processor[code]
	return ep, ok
}

func (ep builtinEntryPoint) Run(ctx vm.Sandbox) {
	ep(ctx)
}

func stub(ctx vm.Sandbox) {
	reqId := ctx.GetRequestID()
	ctx.GetLog().Debugw("run builtInProcessor: not implemented",
		"request code", ctx.GetRequestCode(),
		"addr", ctx.GetAddress().String(),
		"ts", ctx.GetTimestamp(),
		"state index", ctx.GetStateIndex(),
		"req", reqId.String(),
	)
}

func nopRequest(ctx vm.Sandbox) {
	stub(ctx)
}

func setMinimumRewardRequest(ctx vm.Sandbox) {
	stub(ctx)
}

func setDescriptionRequest(ctx vm.Sandbox) {
	stub(ctx)
}