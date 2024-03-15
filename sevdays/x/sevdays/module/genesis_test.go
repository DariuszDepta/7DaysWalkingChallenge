package sevdays_test

import (
	"testing"

	"github.com/stretchr/testify/require"
	keepertest "sevdays/testutil/keeper"
	"sevdays/testutil/nullify"
	"sevdays/x/sevdays/module"
	"sevdays/x/sevdays/types"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),

		// this line is used by starport scaffolding # genesis/test/state
	}

	k, ctx := keepertest.SevdaysKeeper(t)
	sevdays.InitGenesis(ctx, k, genesisState)
	got := sevdays.ExportGenesis(ctx, k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	// this line is used by starport scaffolding # genesis/test/assert
}
