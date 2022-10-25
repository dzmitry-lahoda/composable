import { useStore } from "@/stores/root";
import { useEffect } from "react";
import BigNumber from "bignumber.js";
import { callbackGate, fromChainIdUnit, unwrapNumberOrHex } from "shared";
import { useTransfer } from "@/defi/polkadot/hooks/useTransfer";
import { getAssetOnChainId } from "@/defi/polkadot/Assets";

export const useExistentialDeposit = () => {
  const { from, balance, to, account, fromProvider } = useTransfer();

  const tokenId = useStore((state) => state.transfers.selectedToken);
  const updateFeeToken = useStore((state) => state.transfers.updateFeeToken);

  const getFeeToken = useStore((state) => state.transfers.getFeeToken);

  const { native, assets } = useStore(
    ({ substrateBalances }) => substrateBalances.assets[from]
  );

  const { updateExistentialDeposit, existentialDeposit } = useStore(
    (state) => state.transfers
  );

  const { parachainApi } = fromProvider;

  /**
   * Fetch transfer token based on user config (only on karura and picasso)
   * for kusama transfer token is ksm
   *
   * If no transfer fee token is specified, fallback to native token
   */
  useEffect(() => {
    switch (from) {
      case "karura":
        // @see https://wiki.acala.network/get-started/get-started/karura-account
        const karuraEdMap: {
          [key in string] : BigNumber
        } = {
          "kusd": new BigNumber(0.01),
          "ausd": new BigNumber(0.01),
          "kar": new BigNumber(0.1),
          "ksm": new BigNumber(0.0001),
        };
        const ed = tokenId in karuraEdMap ? karuraEdMap[tokenId] : new BigNumber(1);
        const assetOnChain = getAssetOnChainId("karura", tokenId);
        if (assetOnChain) {
          updateFeeToken(assetOnChain);
        }
        updateExistentialDeposit(ed);
        break;
      case "picasso":
        callbackGate(
          async function updateTransferFeeRequirements(api, address) {
            const result: any = await api.query.assetTxPayment.paymentAssets(
              api.createType("AccountId32", address)
            );
            if (result.isNone && tokenId) {
              // Fetch native asset's ED
              const ed = await api.query.currencyFactory.assetEd(
                assets[tokenId].meta.supportedNetwork[from]
              );
              const existentialString = ed.toString();
              const existentialValue = fromChainIdUnit(
                new BigNumber(existentialString)
              );
              updateExistentialDeposit(
                existentialValue.isNaN() ? new BigNumber(0) : existentialValue
              );
              updateFeeToken(1);
              return;
            }
            const [assetId, ed] = result.toJSON();
            updateExistentialDeposit(fromChainIdUnit(unwrapNumberOrHex(ed)));
            updateFeeToken(Number(assetId));
          },
          parachainApi,
          account?.address
        );
        break;
      case "kusama":
        callbackGate(async (api) => {
          const ed = api.consts.balances.existentialDeposit.toString();
          updateExistentialDeposit(fromChainIdUnit(unwrapNumberOrHex(ed)));
          updateFeeToken(Number(1));
        }, parachainApi);
        break;
      default:
        console.log(from);
        break;
    }
  }, [
    from,
    to,
    account,
    tokenId,
    parachainApi,
    updateExistentialDeposit,
    updateFeeToken,
    assets,
  ]);

  return {
    balance,
    tokenId,
    from,
    to,
    assets,
    native,
    existentialDeposit,
    feeToken: getFeeToken(from),
  };
};
