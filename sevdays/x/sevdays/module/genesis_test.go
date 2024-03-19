package sevdays_test

import (
	"testing"

	keepertest "sevdays/testutil/keeper"
	"sevdays/testutil/nullify"
	sevdays "sevdays/x/sevdays/module"
	"sevdays/x/sevdays/types"

	"github.com/stretchr/testify/require"
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
