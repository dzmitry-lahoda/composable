import { useCallback } from "react";
import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "../../../updaters/constants";

export function useClaimOffer() {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  const cancel = useCallback(
    async (assetId: number) => {
      if (!parachainApi) return null;
      const result = await parachainApi.tx.vesting.claim(assetId);
      return result.unwrap();
    },
    [parachainApi]
  );

  return cancel;
}
