package keeper_test

import (
	"testing"

	"github.com/stretchr/testify/require"

	keepertest "sevdays/testutil/keeper"
	"sevdays/x/sevdays/types"
)

func TestGetParams(t *testing.T) {
	k, ctx := keepertest.SevdaysKeeper(t)
	params := types.DefaultParams()

	require.NoError(t, k.SetParams(ctx, params))
	require.EqualValues(t, params, k.GetParams(ctx))
}
