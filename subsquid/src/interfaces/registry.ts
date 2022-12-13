// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/types/registry";

import type {
  CommonMaxStringSize,
  CommonMosaicRemoteAssetId,
  ComposableSupportCollectionsVecBoundedBiBoundedVec,
  ComposableSupportEcdsaSignature,
  ComposableSupportEthereumAddress,
  ComposableTraitsAccountProxyProxyDefinition,
  ComposableTraitsAccountProxyProxyType,
  ComposableTraitsAssetsBasicAssetMetadata,
  ComposableTraitsBondedFinanceBondDuration,
  ComposableTraitsBondedFinanceBondOffer,
  ComposableTraitsBondedFinanceBondOfferReward,
  ComposableTraitsCallFilterCallFilterEntry,
  ComposableTraitsDefiCurrencyPairCurrencyId,
  ComposableTraitsDefiCurrencyPairU128,
  ComposableTraitsDefiSellCurrencyId,
  ComposableTraitsDefiSellU128,
  ComposableTraitsDefiTake,
  ComposableTraitsDexConstantProductPoolInfo,
  ComposableTraitsDexDexRoute,
  ComposableTraitsDexFee,
  ComposableTraitsDexFeeConfig,
  ComposableTraitsDexLiquidityBootstrappingPoolInfo,
  ComposableTraitsDexSale,
  ComposableTraitsDexStableSwapPoolInfo,
  ComposableTraitsGovernanceSignedRawOrigin,
  ComposableTraitsLendingCreateInput,
  ComposableTraitsLendingMarketConfig,
  ComposableTraitsLendingMathCurveModel,
  ComposableTraitsLendingMathDoubleExponentModel,
  ComposableTraitsLendingMathDynamicPIDControllerModel,
  ComposableTraitsLendingMathInterestRateModel,
  ComposableTraitsLendingMathJumpModel,
  ComposableTraitsLendingRepayStrategy,
  ComposableTraitsLendingUpdateInput,
  ComposableTraitsOraclePrice,
  ComposableTraitsOracleRewardTracker,
  ComposableTraitsStakingLock,
  ComposableTraitsStakingLockLockConfig,
  ComposableTraitsStakingReward,
  ComposableTraitsStakingRewardConfig,
  ComposableTraitsStakingRewardPool,
  ComposableTraitsStakingRewardPoolConfiguration,
  ComposableTraitsStakingRewardRate,
  ComposableTraitsStakingRewardRatePeriod,
  ComposableTraitsStakingRewardUpdate,
  ComposableTraitsStakingStake,
  ComposableTraitsTimeLinearDecrease,
  ComposableTraitsTimeStairstepExponentialDecrease,
  ComposableTraitsTimeTimeReleaseFunction,
  ComposableTraitsVaultDeposit,
  ComposableTraitsVaultVaultConfig,
  ComposableTraitsVestingVestingSchedule,
  ComposableTraitsVestingVestingScheduleIdSet,
  ComposableTraitsVestingVestingScheduleInfo,
  ComposableTraitsVestingVestingWindow,
  ComposableTraitsXcmAssetsForeignMetadata,
  ComposableTraitsXcmAssetsXcmAssetLocation,
  ComposableTraitsXcmCumulusMethodId,
  ComposableTraitsXcmXcmSellRequest,
  ComposableTraitsXcmXcmSellRequestTransactConfiguration,
  ComposableTraitsXcmXcmTransactConfiguration,
  CumulusPalletDmpQueueCall,
  CumulusPalletDmpQueueConfigData,
  CumulusPalletDmpQueueError,
  CumulusPalletDmpQueueEvent,
  CumulusPalletDmpQueuePageIndexData,
  CumulusPalletParachainSystemCall,
  CumulusPalletParachainSystemError,
  CumulusPalletParachainSystemEvent,
  CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot,
  CumulusPalletXcmCall,
  CumulusPalletXcmError,
  CumulusPalletXcmEvent,
  CumulusPalletXcmOrigin,
  CumulusPalletXcmpQueueCall,
  CumulusPalletXcmpQueueError,
  CumulusPalletXcmpQueueEvent,
  CumulusPalletXcmpQueueInboundChannelDetails,
  CumulusPalletXcmpQueueInboundState,
  CumulusPalletXcmpQueueOutboundChannelDetails,
  CumulusPalletXcmpQueueOutboundState,
  CumulusPalletXcmpQueueQueueConfigData,
  CumulusPrimitivesParachainInherentParachainInherentData,
  DaliRuntimeMaxHopsCount,
  DaliRuntimeOpaqueSessionKeys,
  DaliRuntimeOriginCaller,
  DaliRuntimeRuntime,
  FrameSupportDispatchRawOrigin,
  FrameSupportPalletId,
  FrameSupportScheduleLookupError,
  FrameSupportScheduleMaybeHashed,
  FrameSupportTokensMiscBalanceStatus,
  FrameSupportWeightsDispatchClass,
  FrameSupportWeightsDispatchInfo,
  FrameSupportWeightsPays,
  FrameSupportWeightsPerDispatchClassU32,
  FrameSupportWeightsPerDispatchClassU64,
  FrameSupportWeightsPerDispatchClassWeightsPerClass,
  FrameSupportWeightsRuntimeDbWeight,
  FrameSystemAccountInfo,
  FrameSystemCall,
  FrameSystemError,
  FrameSystemEvent,
  FrameSystemEventRecord,
  FrameSystemExtensionsCheckGenesis,
  FrameSystemExtensionsCheckNonZeroSender,
  FrameSystemExtensionsCheckNonce,
  FrameSystemExtensionsCheckSpecVersion,
  FrameSystemExtensionsCheckTxVersion,
  FrameSystemExtensionsCheckWeight,
  FrameSystemLastRuntimeUpgradeInfo,
  FrameSystemLimitsBlockLength,
  FrameSystemLimitsBlockWeights,
  FrameSystemLimitsWeightsPerClass,
  FrameSystemPhase,
  IbcTraitOpenChannelParams,
  IbcTransferCall,
  IbcTransferError,
  IbcTransferEvent,
  IbcTransferPalletParams,
  IbcTransferTransferParams,
  OrmlTokensAccountData,
  OrmlTokensBalanceLock,
  OrmlTokensModuleCall,
  OrmlTokensModuleError,
  OrmlTokensModuleEvent,
  OrmlTokensReserveData,
  OrmlUnknownTokensModuleCall,
  OrmlUnknownTokensModuleError,
  OrmlUnknownTokensModuleEvent,
  OrmlXtokensModuleCall,
  OrmlXtokensModuleError,
  OrmlXtokensModuleEvent,
  PalletAccountProxyAnnouncement,
  PalletAccountProxyCall,
  PalletAccountProxyError,
  PalletAccountProxyEvent,
  PalletAssetTxPaymentCall,
  PalletAssetTxPaymentChargeAssetTxPayment,
  PalletAssetsCall,
  PalletAssetsError,
  PalletAssetsRegistryCall,
  PalletAssetsRegistryError,
  PalletAssetsRegistryEvent,
  PalletAuthorshipCall,
  PalletAuthorshipError,
  PalletAuthorshipUncleEntryItem,
  PalletBalancesAccountData,
  PalletBalancesBalanceLock,
  PalletBalancesCall,
  PalletBalancesError,
  PalletBalancesEvent,
  PalletBalancesReasons,
  PalletBalancesReleases,
  PalletBalancesReserveData,
  PalletBondedFinanceCall,
  PalletBondedFinanceError,
  PalletBondedFinanceEvent,
  PalletCallFilterCall,
  PalletCallFilterError,
  PalletCallFilterEvent,
  PalletCollatorSelectionCall,
  PalletCollatorSelectionCandidateInfo,
  PalletCollatorSelectionError,
  PalletCollatorSelectionEvent,
  PalletCollectiveCall,
  PalletCollectiveError,
  PalletCollectiveEvent,
  PalletCollectiveRawOrigin,
  PalletCollectiveVotes,
  PalletCosmwasmCall,
  PalletCosmwasmCodeInfo,
  PalletCosmwasmContractInfo,
  PalletCosmwasmEntryPoint,
  PalletCosmwasmError,
  PalletCosmwasmEvent,
  PalletCrowdloanRewardsCall,
  PalletCrowdloanRewardsError,
  PalletCrowdloanRewardsEvent,
  PalletCrowdloanRewardsModelsProof,
  PalletCrowdloanRewardsModelsRemoteAccount,
  PalletCrowdloanRewardsModelsReward,
  PalletCurrencyFactoryCall,
  PalletCurrencyFactoryError,
  PalletCurrencyFactoryEvent,
  PalletCurrencyFactoryRanges,
  PalletDemocracyCall,
  PalletDemocracyConviction,
  PalletDemocracyDelegations,
  PalletDemocracyError,
  PalletDemocracyEvent,
  PalletDemocracyPreimageStatus,
  PalletDemocracyReferendumInfo,
  PalletDemocracyReferendumStatus,
  PalletDemocracyReleases,
  PalletDemocracyTally,
  PalletDemocracyVoteAccountVote,
  PalletDemocracyVotePriorLock,
  PalletDemocracyVoteThreshold,
  PalletDemocracyVoteVoting,
  PalletDexRouterCall,
  PalletDexRouterError,
  PalletDexRouterEvent,
  PalletDutchAuctionCall,
  PalletDutchAuctionEdContext,
  PalletDutchAuctionError,
  PalletDutchAuctionEvent,
  PalletDutchAuctionSellOrder,
  PalletDutchAuctionTakeOrder,
  PalletFnftError,
  PalletFnftEvent,
  PalletGovernanceRegistryCall,
  PalletGovernanceRegistryError,
  PalletGovernanceRegistryEvent,
  PalletIbcAny,
  PalletIbcCall,
  PalletIbcConnectionParams,
  PalletIbcError,
  PalletIbcErrorsIbcError,
  PalletIbcEvent,
  PalletIbcEventsIbcEvent,
  PalletIbcIbcConsensusState,
  PalletIbcPingCall,
  PalletIbcPingError,
  PalletIbcPingEvent,
  PalletIbcPingSendPingParams,
  PalletIdentityBitFlags,
  PalletIdentityCall,
  PalletIdentityError,
  PalletIdentityEvent,
  PalletIdentityIdentityField,
  PalletIdentityIdentityInfo,
  PalletIdentityJudgement,
  PalletIdentityRegistrarInfo,
  PalletIdentityRegistration,
  PalletIndicesCall,
  PalletIndicesError,
  PalletIndicesEvent,
  PalletLendingCall,
  PalletLendingError,
  PalletLendingEvent,
  PalletLiquidationsCall,
  PalletLiquidationsError,
  PalletLiquidationsEvent,
  PalletLiquidationsLiquidationStrategyConfiguration,
  PalletMembershipCall,
  PalletMembershipError,
  PalletMembershipEvent,
  PalletMosaicAmmSwapInfo,
  PalletMosaicAssetInfo,
  PalletMosaicCall,
  PalletMosaicDecayBudgetPenaltyDecayer,
  PalletMosaicDecayLinearDecay,
  PalletMosaicError,
  PalletMosaicEvent,
  PalletMosaicNetworkInfo,
  PalletMosaicRelayerNext,
  PalletMosaicRelayerRelayerConfig,
  PalletMosaicRelayerStaleRelayer,
  PalletMosaicRemoteAmm,
  PalletMultisigCall,
  PalletMultisigError,
  PalletMultisigEvent,
  PalletMultisigMultisig,
  PalletMultisigTimepoint,
  PalletOracleAssetInfo,
  PalletOracleCall,
  PalletOracleError,
  PalletOracleEvent,
  PalletOraclePrePrice,
  PalletOracleWithdraw,
  PalletPabloCall,
  PalletPabloError,
  PalletPabloEvent,
  PalletPabloPoolConfiguration,
  PalletPabloPoolInitConfiguration,
  PalletPabloPriceCumulative,
  PalletPabloTimeWeightedAveragePrice,
  PalletPreimageCall,
  PalletPreimageError,
  PalletPreimageEvent,
  PalletPreimageRequestStatus,
  PalletSchedulerCall,
  PalletSchedulerError,
  PalletSchedulerEvent,
  PalletSchedulerScheduledV3,
  PalletSessionCall,
  PalletSessionError,
  PalletSessionEvent,
  PalletStakingRewardsCall,
  PalletStakingRewardsError,
  PalletStakingRewardsEvent,
  PalletStakingRewardsRewardAccumulationHookError,
  PalletSudoCall,
  PalletSudoError,
  PalletSudoEvent,
  PalletTimestampCall,
  PalletTransactionPaymentEvent,
  PalletTransactionPaymentReleases,
  PalletTreasuryCall,
  PalletTreasuryError,
  PalletTreasuryEvent,
  PalletTreasuryProposal,
  PalletUtilityCall,
  PalletUtilityError,
  PalletUtilityEvent,
  PalletVaultCall,
  PalletVaultCapabilities,
  PalletVaultError,
  PalletVaultEvent,
  PalletVaultModelsStrategyOverview,
  PalletVaultModelsVaultInfo,
  PalletVestingModuleCall,
  PalletVestingModuleError,
  PalletVestingModuleEvent,
  PalletXcmCall,
  PalletXcmError,
  PalletXcmEvent,
  PalletXcmOrigin,
  PalletXcmQueryStatus,
  PalletXcmVersionMigrationStage,
  ParachainInfoCall,
  PolkadotCorePrimitivesInboundDownwardMessage,
  PolkadotCorePrimitivesInboundHrmpMessage,
  PolkadotCorePrimitivesOutboundHrmpMessage,
  PolkadotParachainPrimitivesXcmpMessageFormat,
  PolkadotPrimitivesV2AbridgedHostConfiguration,
  PolkadotPrimitivesV2AbridgedHrmpChannel,
  PolkadotPrimitivesV2PersistedValidationData,
  PolkadotPrimitivesV2UpgradeRestriction,
  SpConsensusAuraSr25519AppSr25519Public,
  SpCoreCryptoKeyTypeId,
  SpCoreEcdsaSignature,
  SpCoreEd25519Signature,
  SpCoreSr25519Public,
  SpCoreSr25519Signature,
  SpCoreVoid,
  SpRuntimeArithmeticError,
  SpRuntimeBlakeTwo256,
  SpRuntimeDigest,
  SpRuntimeDigestDigestItem,
  SpRuntimeDispatchError,
  SpRuntimeHeader,
  SpRuntimeModuleError,
  SpRuntimeMultiSignature,
  SpRuntimeTokenError,
  SpRuntimeTransactionalError,
  SpTrieStorageProof,
  SpVersionRuntimeVersion,
  XcmDoubleEncoded,
  XcmV0Junction,
  XcmV0JunctionBodyId,
  XcmV0JunctionBodyPart,
  XcmV0JunctionNetworkId,
  XcmV0MultiAsset,
  XcmV0MultiLocation,
  XcmV0Order,
  XcmV0OriginKind,
  XcmV0Response,
  XcmV0Xcm,
  XcmV1Junction,
  XcmV1MultiAsset,
  XcmV1MultiLocation,
  XcmV1MultiassetAssetId,
  XcmV1MultiassetAssetInstance,
  XcmV1MultiassetFungibility,
  XcmV1MultiassetMultiAssetFilter,
  XcmV1MultiassetMultiAssets,
  XcmV1MultiassetWildFungibility,
  XcmV1MultiassetWildMultiAsset,
  XcmV1MultilocationJunctions,
  XcmV1Order,
  XcmV1Response,
  XcmV1Xcm,
  XcmV2Instruction,
  XcmV2Response,
  XcmV2TraitsError,
  XcmV2TraitsOutcome,
  XcmV2WeightLimit,
  XcmV2Xcm,
  XcmVersionedMultiAsset,
  XcmVersionedMultiAssets,
  XcmVersionedMultiLocation,
  XcmVersionedResponse,
  XcmVersionedXcm,
} from "@polkadot/types/lookup";

declare module "@polkadot/types/types/registry" {
  interface InterfaceTypes {
    CommonMaxStringSize: CommonMaxStringSize;
    CommonMosaicRemoteAssetId: CommonMosaicRemoteAssetId;
    ComposableSupportCollectionsVecBoundedBiBoundedVec: ComposableSupportCollectionsVecBoundedBiBoundedVec;
    ComposableSupportEcdsaSignature: ComposableSupportEcdsaSignature;
    ComposableSupportEthereumAddress: ComposableSupportEthereumAddress;
    ComposableTraitsAccountProxyProxyDefinition: ComposableTraitsAccountProxyProxyDefinition;
    ComposableTraitsAccountProxyProxyType: ComposableTraitsAccountProxyProxyType;
    ComposableTraitsAssetsBasicAssetMetadata: ComposableTraitsAssetsBasicAssetMetadata;
    ComposableTraitsBondedFinanceBondDuration: ComposableTraitsBondedFinanceBondDuration;
    ComposableTraitsBondedFinanceBondOffer: ComposableTraitsBondedFinanceBondOffer;
    ComposableTraitsBondedFinanceBondOfferReward: ComposableTraitsBondedFinanceBondOfferReward;
    ComposableTraitsCallFilterCallFilterEntry: ComposableTraitsCallFilterCallFilterEntry;
    ComposableTraitsDefiCurrencyPairCurrencyId: ComposableTraitsDefiCurrencyPairCurrencyId;
    ComposableTraitsDefiCurrencyPairU128: ComposableTraitsDefiCurrencyPairU128;
    ComposableTraitsDefiSellCurrencyId: ComposableTraitsDefiSellCurrencyId;
    ComposableTraitsDefiSellU128: ComposableTraitsDefiSellU128;
    ComposableTraitsDefiTake: ComposableTraitsDefiTake;
    ComposableTraitsDexConstantProductPoolInfo: ComposableTraitsDexConstantProductPoolInfo;
    ComposableTraitsDexDexRoute: ComposableTraitsDexDexRoute;
    ComposableTraitsDexFee: ComposableTraitsDexFee;
    ComposableTraitsDexFeeConfig: ComposableTraitsDexFeeConfig;
    ComposableTraitsDexLiquidityBootstrappingPoolInfo: ComposableTraitsDexLiquidityBootstrappingPoolInfo;
    ComposableTraitsDexSale: ComposableTraitsDexSale;
    ComposableTraitsDexStableSwapPoolInfo: ComposableTraitsDexStableSwapPoolInfo;
    ComposableTraitsGovernanceSignedRawOrigin: ComposableTraitsGovernanceSignedRawOrigin;
    ComposableTraitsLendingCreateInput: ComposableTraitsLendingCreateInput;
    ComposableTraitsLendingMarketConfig: ComposableTraitsLendingMarketConfig;
    ComposableTraitsLendingMathCurveModel: ComposableTraitsLendingMathCurveModel;
    ComposableTraitsLendingMathDoubleExponentModel: ComposableTraitsLendingMathDoubleExponentModel;
    ComposableTraitsLendingMathDynamicPIDControllerModel: ComposableTraitsLendingMathDynamicPIDControllerModel;
    ComposableTraitsLendingMathInterestRateModel: ComposableTraitsLendingMathInterestRateModel;
    ComposableTraitsLendingMathJumpModel: ComposableTraitsLendingMathJumpModel;
    ComposableTraitsLendingRepayStrategy: ComposableTraitsLendingRepayStrategy;
    ComposableTraitsLendingUpdateInput: ComposableTraitsLendingUpdateInput;
    ComposableTraitsOraclePrice: ComposableTraitsOraclePrice;
    ComposableTraitsOracleRewardTracker: ComposableTraitsOracleRewardTracker;
    ComposableTraitsStakingLock: ComposableTraitsStakingLock;
    ComposableTraitsStakingLockLockConfig: ComposableTraitsStakingLockLockConfig;
    ComposableTraitsStakingReward: ComposableTraitsStakingReward;
    ComposableTraitsStakingRewardConfig: ComposableTraitsStakingRewardConfig;
    ComposableTraitsStakingRewardPool: ComposableTraitsStakingRewardPool;
    ComposableTraitsStakingRewardPoolConfiguration: ComposableTraitsStakingRewardPoolConfiguration;
    ComposableTraitsStakingRewardRate: ComposableTraitsStakingRewardRate;
    ComposableTraitsStakingRewardRatePeriod: ComposableTraitsStakingRewardRatePeriod;
    ComposableTraitsStakingRewardUpdate: ComposableTraitsStakingRewardUpdate;
    ComposableTraitsStakingStake: ComposableTraitsStakingStake;
    ComposableTraitsTimeLinearDecrease: ComposableTraitsTimeLinearDecrease;
    ComposableTraitsTimeStairstepExponentialDecrease: ComposableTraitsTimeStairstepExponentialDecrease;
    ComposableTraitsTimeTimeReleaseFunction: ComposableTraitsTimeTimeReleaseFunction;
    ComposableTraitsVaultDeposit: ComposableTraitsVaultDeposit;
    ComposableTraitsVaultVaultConfig: ComposableTraitsVaultVaultConfig;
    ComposableTraitsVestingVestingSchedule: ComposableTraitsVestingVestingSchedule;
    ComposableTraitsVestingVestingScheduleIdSet: ComposableTraitsVestingVestingScheduleIdSet;
    ComposableTraitsVestingVestingScheduleInfo: ComposableTraitsVestingVestingScheduleInfo;
    ComposableTraitsVestingVestingWindow: ComposableTraitsVestingVestingWindow;
    ComposableTraitsXcmAssetsForeignMetadata: ComposableTraitsXcmAssetsForeignMetadata;
    ComposableTraitsXcmAssetsXcmAssetLocation: ComposableTraitsXcmAssetsXcmAssetLocation;
    ComposableTraitsXcmCumulusMethodId: ComposableTraitsXcmCumulusMethodId;
    ComposableTraitsXcmXcmSellRequest: ComposableTraitsXcmXcmSellRequest;
    ComposableTraitsXcmXcmSellRequestTransactConfiguration: ComposableTraitsXcmXcmSellRequestTransactConfiguration;
    ComposableTraitsXcmXcmTransactConfiguration: ComposableTraitsXcmXcmTransactConfiguration;
    CumulusPalletDmpQueueCall: CumulusPalletDmpQueueCall;
    CumulusPalletDmpQueueConfigData: CumulusPalletDmpQueueConfigData;
    CumulusPalletDmpQueueError: CumulusPalletDmpQueueError;
    CumulusPalletDmpQueueEvent: CumulusPalletDmpQueueEvent;
    CumulusPalletDmpQueuePageIndexData: CumulusPalletDmpQueuePageIndexData;
    CumulusPalletParachainSystemCall: CumulusPalletParachainSystemCall;
    CumulusPalletParachainSystemError: CumulusPalletParachainSystemError;
    CumulusPalletParachainSystemEvent: CumulusPalletParachainSystemEvent;
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot;
    CumulusPalletXcmCall: CumulusPalletXcmCall;
    CumulusPalletXcmError: CumulusPalletXcmError;
    CumulusPalletXcmEvent: CumulusPalletXcmEvent;
    CumulusPalletXcmOrigin: CumulusPalletXcmOrigin;
    CumulusPalletXcmpQueueCall: CumulusPalletXcmpQueueCall;
    CumulusPalletXcmpQueueError: CumulusPalletXcmpQueueError;
    CumulusPalletXcmpQueueEvent: CumulusPalletXcmpQueueEvent;
    CumulusPalletXcmpQueueInboundChannelDetails: CumulusPalletXcmpQueueInboundChannelDetails;
    CumulusPalletXcmpQueueInboundState: CumulusPalletXcmpQueueInboundState;
    CumulusPalletXcmpQueueOutboundChannelDetails: CumulusPalletXcmpQueueOutboundChannelDetails;
    CumulusPalletXcmpQueueOutboundState: CumulusPalletXcmpQueueOutboundState;
    CumulusPalletXcmpQueueQueueConfigData: CumulusPalletXcmpQueueQueueConfigData;
    CumulusPrimitivesParachainInherentParachainInherentData: CumulusPrimitivesParachainInherentParachainInherentData;
    DaliRuntimeMaxHopsCount: DaliRuntimeMaxHopsCount;
    DaliRuntimeOpaqueSessionKeys: DaliRuntimeOpaqueSessionKeys;
    DaliRuntimeOriginCaller: DaliRuntimeOriginCaller;
    DaliRuntimeRuntime: DaliRuntimeRuntime;
    FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
    FrameSupportPalletId: FrameSupportPalletId;
    FrameSupportScheduleLookupError: FrameSupportScheduleLookupError;
    FrameSupportScheduleMaybeHashed: FrameSupportScheduleMaybeHashed;
    FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
    FrameSupportWeightsDispatchClass: FrameSupportWeightsDispatchClass;
    FrameSupportWeightsDispatchInfo: FrameSupportWeightsDispatchInfo;
    FrameSupportWeightsPays: FrameSupportWeightsPays;
    FrameSupportWeightsPerDispatchClassU32: FrameSupportWeightsPerDispatchClassU32;
    FrameSupportWeightsPerDispatchClassU64: FrameSupportWeightsPerDispatchClassU64;
    FrameSupportWeightsPerDispatchClassWeightsPerClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
    FrameSupportWeightsRuntimeDbWeight: FrameSupportWeightsRuntimeDbWeight;
    FrameSystemAccountInfo: FrameSystemAccountInfo;
    FrameSystemCall: FrameSystemCall;
    FrameSystemError: FrameSystemError;
    FrameSystemEvent: FrameSystemEvent;
    FrameSystemEventRecord: FrameSystemEventRecord;
    FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
    FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
    FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
    FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
    FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
    FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
    FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
    FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
    FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
    FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
    FrameSystemPhase: FrameSystemPhase;
    IbcTraitOpenChannelParams: IbcTraitOpenChannelParams;
    IbcTransferCall: IbcTransferCall;
    IbcTransferError: IbcTransferError;
    IbcTransferEvent: IbcTransferEvent;
    IbcTransferPalletParams: IbcTransferPalletParams;
    IbcTransferTransferParams: IbcTransferTransferParams;
    OrmlTokensAccountData: OrmlTokensAccountData;
    OrmlTokensBalanceLock: OrmlTokensBalanceLock;
    OrmlTokensModuleCall: OrmlTokensModuleCall;
    OrmlTokensModuleError: OrmlTokensModuleError;
    OrmlTokensModuleEvent: OrmlTokensModuleEvent;
    OrmlTokensReserveData: OrmlTokensReserveData;
    OrmlUnknownTokensModuleCall: OrmlUnknownTokensModuleCall;
    OrmlUnknownTokensModuleError: OrmlUnknownTokensModuleError;
    OrmlUnknownTokensModuleEvent: OrmlUnknownTokensModuleEvent;
    OrmlXtokensModuleCall: OrmlXtokensModuleCall;
    OrmlXtokensModuleError: OrmlXtokensModuleError;
    OrmlXtokensModuleEvent: OrmlXtokensModuleEvent;
    PalletAccountProxyAnnouncement: PalletAccountProxyAnnouncement;
    PalletAccountProxyCall: PalletAccountProxyCall;
    PalletAccountProxyError: PalletAccountProxyError;
    PalletAccountProxyEvent: PalletAccountProxyEvent;
    PalletAssetTxPaymentCall: PalletAssetTxPaymentCall;
    PalletAssetTxPaymentChargeAssetTxPayment: PalletAssetTxPaymentChargeAssetTxPayment;
    PalletAssetsCall: PalletAssetsCall;
    PalletAssetsError: PalletAssetsError;
    PalletAssetsRegistryCall: PalletAssetsRegistryCall;
    PalletAssetsRegistryError: PalletAssetsRegistryError;
    PalletAssetsRegistryEvent: PalletAssetsRegistryEvent;
    PalletAuthorshipCall: PalletAuthorshipCall;
    PalletAuthorshipError: PalletAuthorshipError;
    PalletAuthorshipUncleEntryItem: PalletAuthorshipUncleEntryItem;
    PalletBalancesAccountData: PalletBalancesAccountData;
    PalletBalancesBalanceLock: PalletBalancesBalanceLock;
    PalletBalancesCall: PalletBalancesCall;
    PalletBalancesError: PalletBalancesError;
    PalletBalancesEvent: PalletBalancesEvent;
    PalletBalancesReasons: PalletBalancesReasons;
    PalletBalancesReleases: PalletBalancesReleases;
    PalletBalancesReserveData: PalletBalancesReserveData;
    PalletBondedFinanceCall: PalletBondedFinanceCall;
    PalletBondedFinanceError: PalletBondedFinanceError;
    PalletBondedFinanceEvent: PalletBondedFinanceEvent;
    PalletCallFilterCall: PalletCallFilterCall;
    PalletCallFilterError: PalletCallFilterError;
    PalletCallFilterEvent: PalletCallFilterEvent;
    PalletCollatorSelectionCall: PalletCollatorSelectionCall;
    PalletCollatorSelectionCandidateInfo: PalletCollatorSelectionCandidateInfo;
    PalletCollatorSelectionError: PalletCollatorSelectionError;
    PalletCollatorSelectionEvent: PalletCollatorSelectionEvent;
    PalletCollectiveCall: PalletCollectiveCall;
    PalletCollectiveError: PalletCollectiveError;
    PalletCollectiveEvent: PalletCollectiveEvent;
    PalletCollectiveRawOrigin: PalletCollectiveRawOrigin;
    PalletCollectiveVotes: PalletCollectiveVotes;
    PalletCosmwasmCall: PalletCosmwasmCall;
    PalletCosmwasmCodeInfo: PalletCosmwasmCodeInfo;
    PalletCosmwasmContractInfo: PalletCosmwasmContractInfo;
    PalletCosmwasmEntryPoint: PalletCosmwasmEntryPoint;
    PalletCosmwasmError: PalletCosmwasmError;
    PalletCosmwasmEvent: PalletCosmwasmEvent;
    PalletCrowdloanRewardsCall: PalletCrowdloanRewardsCall;
    PalletCrowdloanRewardsError: PalletCrowdloanRewardsError;
    PalletCrowdloanRewardsEvent: PalletCrowdloanRewardsEvent;
    PalletCrowdloanRewardsModelsProof: PalletCrowdloanRewardsModelsProof;
    PalletCrowdloanRewardsModelsRemoteAccount: PalletCrowdloanRewardsModelsRemoteAccount;
    PalletCrowdloanRewardsModelsReward: PalletCrowdloanRewardsModelsReward;
    PalletCurrencyFactoryCall: PalletCurrencyFactoryCall;
    PalletCurrencyFactoryError: PalletCurrencyFactoryError;
    PalletCurrencyFactoryEvent: PalletCurrencyFactoryEvent;
    PalletCurrencyFactoryRanges: PalletCurrencyFactoryRanges;
    PalletDemocracyCall: PalletDemocracyCall;
    PalletDemocracyConviction: PalletDemocracyConviction;
    PalletDemocracyDelegations: PalletDemocracyDelegations;
    PalletDemocracyError: PalletDemocracyError;
    PalletDemocracyEvent: PalletDemocracyEvent;
    PalletDemocracyPreimageStatus: PalletDemocracyPreimageStatus;
    PalletDemocracyReferendumInfo: PalletDemocracyReferendumInfo;
    PalletDemocracyReferendumStatus: PalletDemocracyReferendumStatus;
    PalletDemocracyReleases: PalletDemocracyReleases;
    PalletDemocracyTally: PalletDemocracyTally;
    PalletDemocracyVoteAccountVote: PalletDemocracyVoteAccountVote;
    PalletDemocracyVotePriorLock: PalletDemocracyVotePriorLock;
    PalletDemocracyVoteThreshold: PalletDemocracyVoteThreshold;
    PalletDemocracyVoteVoting: PalletDemocracyVoteVoting;
    PalletDexRouterCall: PalletDexRouterCall;
    PalletDexRouterError: PalletDexRouterError;
    PalletDexRouterEvent: PalletDexRouterEvent;
    PalletDutchAuctionCall: PalletDutchAuctionCall;
    PalletDutchAuctionEdContext: PalletDutchAuctionEdContext;
    PalletDutchAuctionError: PalletDutchAuctionError;
    PalletDutchAuctionEvent: PalletDutchAuctionEvent;
    PalletDutchAuctionSellOrder: PalletDutchAuctionSellOrder;
    PalletDutchAuctionTakeOrder: PalletDutchAuctionTakeOrder;
    PalletFnftError: PalletFnftError;
    PalletFnftEvent: PalletFnftEvent;
    PalletGovernanceRegistryCall: PalletGovernanceRegistryCall;
    PalletGovernanceRegistryError: PalletGovernanceRegistryError;
    PalletGovernanceRegistryEvent: PalletGovernanceRegistryEvent;
    PalletIbcAny: PalletIbcAny;
    PalletIbcCall: PalletIbcCall;
    PalletIbcConnectionParams: PalletIbcConnectionParams;
    PalletIbcError: PalletIbcError;
    PalletIbcErrorsIbcError: PalletIbcErrorsIbcError;
    PalletIbcEvent: PalletIbcEvent;
    PalletIbcEventsIbcEvent: PalletIbcEventsIbcEvent;
    PalletIbcIbcConsensusState: PalletIbcIbcConsensusState;
    PalletIbcPingCall: PalletIbcPingCall;
    PalletIbcPingError: PalletIbcPingError;
    PalletIbcPingEvent: PalletIbcPingEvent;
    PalletIbcPingSendPingParams: PalletIbcPingSendPingParams;
    PalletIdentityBitFlags: PalletIdentityBitFlags;
    PalletIdentityCall: PalletIdentityCall;
    PalletIdentityError: PalletIdentityError;
    PalletIdentityEvent: PalletIdentityEvent;
    PalletIdentityIdentityField: PalletIdentityIdentityField;
    PalletIdentityIdentityInfo: PalletIdentityIdentityInfo;
    PalletIdentityJudgement: PalletIdentityJudgement;
    PalletIdentityRegistrarInfo: PalletIdentityRegistrarInfo;
    PalletIdentityRegistration: PalletIdentityRegistration;
    PalletIndicesCall: PalletIndicesCall;
    PalletIndicesError: PalletIndicesError;
    PalletIndicesEvent: PalletIndicesEvent;
    PalletLendingCall: PalletLendingCall;
    PalletLendingError: PalletLendingError;
    PalletLendingEvent: PalletLendingEvent;
    PalletLiquidationsCall: PalletLiquidationsCall;
    PalletLiquidationsError: PalletLiquidationsError;
    PalletLiquidationsEvent: PalletLiquidationsEvent;
    PalletLiquidationsLiquidationStrategyConfiguration: PalletLiquidationsLiquidationStrategyConfiguration;
    PalletMembershipCall: PalletMembershipCall;
    PalletMembershipError: PalletMembershipError;
    PalletMembershipEvent: PalletMembershipEvent;
    PalletMosaicAmmSwapInfo: PalletMosaicAmmSwapInfo;
    PalletMosaicAssetInfo: PalletMosaicAssetInfo;
    PalletMosaicCall: PalletMosaicCall;
    PalletMosaicDecayBudgetPenaltyDecayer: PalletMosaicDecayBudgetPenaltyDecayer;
    PalletMosaicDecayLinearDecay: PalletMosaicDecayLinearDecay;
    PalletMosaicError: PalletMosaicError;
    PalletMosaicEvent: PalletMosaicEvent;
    PalletMosaicNetworkInfo: PalletMosaicNetworkInfo;
    PalletMosaicRelayerNext: PalletMosaicRelayerNext;
    PalletMosaicRelayerRelayerConfig: PalletMosaicRelayerRelayerConfig;
    PalletMosaicRelayerStaleRelayer: PalletMosaicRelayerStaleRelayer;
    PalletMosaicRemoteAmm: PalletMosaicRemoteAmm;
    PalletMultisigCall: PalletMultisigCall;
    PalletMultisigError: PalletMultisigError;
    PalletMultisigEvent: PalletMultisigEvent;
    PalletMultisigMultisig: PalletMultisigMultisig;
    PalletMultisigTimepoint: PalletMultisigTimepoint;
    PalletOracleAssetInfo: PalletOracleAssetInfo;
    PalletOracleCall: PalletOracleCall;
    PalletOracleError: PalletOracleError;
    PalletOracleEvent: PalletOracleEvent;
    PalletOraclePrePrice: PalletOraclePrePrice;
    PalletOracleWithdraw: PalletOracleWithdraw;
    PalletPabloCall: PalletPabloCall;
    PalletPabloError: PalletPabloError;
    PalletPabloEvent: PalletPabloEvent;
    PalletPabloPoolConfiguration: PalletPabloPoolConfiguration;
    PalletPabloPoolInitConfiguration: PalletPabloPoolInitConfiguration;
    PalletPabloPriceCumulative: PalletPabloPriceCumulative;
    PalletPabloTimeWeightedAveragePrice: PalletPabloTimeWeightedAveragePrice;
    PalletPreimageCall: PalletPreimageCall;
    PalletPreimageError: PalletPreimageError;
    PalletPreimageEvent: PalletPreimageEvent;
    PalletPreimageRequestStatus: PalletPreimageRequestStatus;
    PalletSchedulerCall: PalletSchedulerCall;
    PalletSchedulerError: PalletSchedulerError;
    PalletSchedulerEvent: PalletSchedulerEvent;
    PalletSchedulerScheduledV3: PalletSchedulerScheduledV3;
    PalletSessionCall: PalletSessionCall;
    PalletSessionError: PalletSessionError;
    PalletSessionEvent: PalletSessionEvent;
    PalletStakingRewardsCall: PalletStakingRewardsCall;
    PalletStakingRewardsError: PalletStakingRewardsError;
    PalletStakingRewardsEvent: PalletStakingRewardsEvent;
    PalletStakingRewardsRewardAccumulationHookError: PalletStakingRewardsRewardAccumulationHookError;
    PalletSudoCall: PalletSudoCall;
    PalletSudoError: PalletSudoError;
    PalletSudoEvent: PalletSudoEvent;
    PalletTimestampCall: PalletTimestampCall;
    PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
    PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
    PalletTreasuryCall: PalletTreasuryCall;
    PalletTreasuryError: PalletTreasuryError;
    PalletTreasuryEvent: PalletTreasuryEvent;
    PalletTreasuryProposal: PalletTreasuryProposal;
    PalletUtilityCall: PalletUtilityCall;
    PalletUtilityError: PalletUtilityError;
    PalletUtilityEvent: PalletUtilityEvent;
    PalletVaultCall: PalletVaultCall;
    PalletVaultCapabilities: PalletVaultCapabilities;
    PalletVaultError: PalletVaultError;
    PalletVaultEvent: PalletVaultEvent;
    PalletVaultModelsStrategyOverview: PalletVaultModelsStrategyOverview;
    PalletVaultModelsVaultInfo: PalletVaultModelsVaultInfo;
    PalletVestingModuleCall: PalletVestingModuleCall;
    PalletVestingModuleError: PalletVestingModuleError;
    PalletVestingModuleEvent: PalletVestingModuleEvent;
    PalletXcmCall: PalletXcmCall;
    PalletXcmError: PalletXcmError;
    PalletXcmEvent: PalletXcmEvent;
    PalletXcmOrigin: PalletXcmOrigin;
    PalletXcmQueryStatus: PalletXcmQueryStatus;
    PalletXcmVersionMigrationStage: PalletXcmVersionMigrationStage;
    ParachainInfoCall: ParachainInfoCall;
    PolkadotCorePrimitivesInboundDownwardMessage: PolkadotCorePrimitivesInboundDownwardMessage;
    PolkadotCorePrimitivesInboundHrmpMessage: PolkadotCorePrimitivesInboundHrmpMessage;
    PolkadotCorePrimitivesOutboundHrmpMessage: PolkadotCorePrimitivesOutboundHrmpMessage;
    PolkadotParachainPrimitivesXcmpMessageFormat: PolkadotParachainPrimitivesXcmpMessageFormat;
    PolkadotPrimitivesV2AbridgedHostConfiguration: PolkadotPrimitivesV2AbridgedHostConfiguration;
    PolkadotPrimitivesV2AbridgedHrmpChannel: PolkadotPrimitivesV2AbridgedHrmpChannel;
    PolkadotPrimitivesV2PersistedValidationData: PolkadotPrimitivesV2PersistedValidationData;
    PolkadotPrimitivesV2UpgradeRestriction: PolkadotPrimitivesV2UpgradeRestriction;
    SpConsensusAuraSr25519AppSr25519Public: SpConsensusAuraSr25519AppSr25519Public;
    SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
    SpCoreEcdsaSignature: SpCoreEcdsaSignature;
    SpCoreEd25519Signature: SpCoreEd25519Signature;
    SpCoreSr25519Public: SpCoreSr25519Public;
    SpCoreSr25519Signature: SpCoreSr25519Signature;
    SpCoreVoid: SpCoreVoid;
    SpRuntimeArithmeticError: SpRuntimeArithmeticError;
    SpRuntimeBlakeTwo256: SpRuntimeBlakeTwo256;
    SpRuntimeDigest: SpRuntimeDigest;
    SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
    SpRuntimeDispatchError: SpRuntimeDispatchError;
    SpRuntimeHeader: SpRuntimeHeader;
    SpRuntimeModuleError: SpRuntimeModuleError;
    SpRuntimeMultiSignature: SpRuntimeMultiSignature;
    SpRuntimeTokenError: SpRuntimeTokenError;
    SpRuntimeTransactionalError: SpRuntimeTransactionalError;
    SpTrieStorageProof: SpTrieStorageProof;
    SpVersionRuntimeVersion: SpVersionRuntimeVersion;
    XcmDoubleEncoded: XcmDoubleEncoded;
    XcmV0Junction: XcmV0Junction;
    XcmV0JunctionBodyId: XcmV0JunctionBodyId;
    XcmV0JunctionBodyPart: XcmV0JunctionBodyPart;
    XcmV0JunctionNetworkId: XcmV0JunctionNetworkId;
    XcmV0MultiAsset: XcmV0MultiAsset;
    XcmV0MultiLocation: XcmV0MultiLocation;
    XcmV0Order: XcmV0Order;
    XcmV0OriginKind: XcmV0OriginKind;
    XcmV0Response: XcmV0Response;
    XcmV0Xcm: XcmV0Xcm;
    XcmV1Junction: XcmV1Junction;
    XcmV1MultiAsset: XcmV1MultiAsset;
    XcmV1MultiLocation: XcmV1MultiLocation;
    XcmV1MultiassetAssetId: XcmV1MultiassetAssetId;
    XcmV1MultiassetAssetInstance: XcmV1MultiassetAssetInstance;
    XcmV1MultiassetFungibility: XcmV1MultiassetFungibility;
    XcmV1MultiassetMultiAssetFilter: XcmV1MultiassetMultiAssetFilter;
    XcmV1MultiassetMultiAssets: XcmV1MultiassetMultiAssets;
    XcmV1MultiassetWildFungibility: XcmV1MultiassetWildFungibility;
    XcmV1MultiassetWildMultiAsset: XcmV1MultiassetWildMultiAsset;
    XcmV1MultilocationJunctions: XcmV1MultilocationJunctions;
    XcmV1Order: XcmV1Order;
    XcmV1Response: XcmV1Response;
    XcmV1Xcm: XcmV1Xcm;
    XcmV2Instruction: XcmV2Instruction;
    XcmV2Response: XcmV2Response;
    XcmV2TraitsError: XcmV2TraitsError;
    XcmV2TraitsOutcome: XcmV2TraitsOutcome;
    XcmV2WeightLimit: XcmV2WeightLimit;
    XcmV2Xcm: XcmV2Xcm;
    XcmVersionedMultiAsset: XcmVersionedMultiAsset;
    XcmVersionedMultiAssets: XcmVersionedMultiAssets;
    XcmVersionedMultiLocation: XcmVersionedMultiLocation;
    XcmVersionedResponse: XcmVersionedResponse;
    XcmVersionedXcm: XcmVersionedXcm;
  } // InterfaceTypes
} // declare module
