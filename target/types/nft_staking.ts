export type NftStaking = {
  "version": "0.1.0",
  "name": "nft_staking",
  "instructions": [
    {
      "name": "initializeGlobal",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "globalName",
          "type": "string"
        },
        {
          "name": "nftCreator",
          "type": "publicKey"
        },
        {
          "name": "rewardTokenMint",
          "type": "publicKey"
        },
        {
          "name": "traitRates",
          "type": {
            "vec": "u64"
          }
        },
        {
          "name": "traitNames",
          "type": {
            "vec": "string"
          }
        },
        {
          "name": "normalRate",
          "type": "u64"
        },
        {
          "name": "lockDurations",
          "type": "bytes"
        },
        {
          "name": "lockRates",
          "type": {
            "vec": "u64"
          }
        },
        {
          "name": "custodial",
          "type": "bool"
        }
      ],
      "returns": null
    },
    {
      "name": "updateAdmin",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "newAdmin",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "updateGlobal",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "nftCreator",
          "type": "publicKey"
        },
        {
          "name": "rewardTokenMint",
          "type": "publicKey"
        },
        {
          "name": "traitRates",
          "type": {
            "vec": "u64"
          }
        },
        {
          "name": "traitNames",
          "type": {
            "vec": "string"
          }
        },
        {
          "name": "normalRate",
          "type": "u64"
        },
        {
          "name": "lockDurations",
          "type": "bytes"
        },
        {
          "name": "lockRates",
          "type": {
            "vec": "u64"
          }
        },
        {
          "name": "custodial",
          "type": "bool"
        }
      ],
      "returns": null
    },
    {
      "name": "initializeFixedPool",
      "accounts": [
        {
          "name": "userFixedPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [],
      "returns": null
    },
    {
      "name": "stakeNftToFixed",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userFixedPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultPda",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "edition",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mintMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "lockPeriod",
          "type": "u8"
        },
        {
          "name": "role",
          "type": "string"
        },
        {
          "name": "model",
          "type": "u64"
        }
      ],
      "returns": null
    },
    {
      "name": "withdrawNftFromFixed",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userFixedPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPda",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "edition",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "vaultStakeBump",
          "type": "u8"
        }
      ],
      "returns": null
    },
    {
      "name": "claimRewardAll",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userFixedPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRewardAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        }
      ],
      "returns": null
    },
    {
      "name": "claimReward",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userFixedPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRewardAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        }
      ],
      "returns": null
    },
    {
      "name": "withdrawReward",
      "accounts": [
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimerRewardAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ],
      "returns": null
    }
  ],
  "accounts": [
    {
      "name": "globalPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "nftCreator",
            "type": "publicKey"
          },
          {
            "name": "rewardTokenMint",
            "type": "publicKey"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "traitRates",
            "type": {
              "vec": "u64"
            }
          },
          {
            "name": "traitNames",
            "type": {
              "vec": "string"
            }
          },
          {
            "name": "normalRate",
            "type": "u64"
          },
          {
            "name": "lockDurations",
            "type": "bytes"
          },
          {
            "name": "lockRates",
            "type": {
              "vec": "u64"
            }
          },
          {
            "name": "custodial",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "userPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "itemCount",
            "type": "u64"
          },
          {
            "name": "items",
            "type": {
              "array": [
                {
                  "defined": "StakedNFT"
                },
                150
              ]
            }
          },
          {
            "name": "rewardTime",
            "type": "i64"
          },
          {
            "name": "pendingReward",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "StakedNFT",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "nftAddr",
            "type": "publicKey"
          },
          {
            "name": "stakeTime",
            "type": "i64"
          },
          {
            "name": "rewardTime",
            "type": "i64"
          },
          {
            "name": "lockTime",
            "type": "i64"
          },
          {
            "name": "rate",
            "type": "i64"
          },
          {
            "name": "model",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidUserPool",
      "msg": "Invalid User Pool"
    },
    {
      "code": 6001,
      "name": "InvalidCollection",
      "msg": "Invalid Collection"
    },
    {
      "code": 6002,
      "name": "InvalidAdmin",
      "msg": "Invalid User Pool"
    },
    {
      "code": 6003,
      "name": "InvalidPoolError",
      "msg": "Invalid pool number"
    },
    {
      "code": 6004,
      "name": "InvalidNFTAddress",
      "msg": "No Matching NFT to withdraw"
    },
    {
      "code": 6005,
      "name": "InvalidOwner",
      "msg": "NFT Owner key mismatch"
    },
    {
      "code": 6006,
      "name": "InvalidWithdrawTime",
      "msg": "Staking Locked Now"
    },
    {
      "code": 6007,
      "name": "IndexOverflow",
      "msg": "Withdraw NFT Index OverFlow"
    },
    {
      "code": 6008,
      "name": "BeforeLockTime",
      "msg": "You can't Unstake Before LockTime"
    },
    {
      "code": 6009,
      "name": "LackLamports",
      "msg": "Insufficient Lamports"
    },
    {
      "code": 6010,
      "name": "MetadataCreatorParseError",
      "msg": "Can't Parse The NFT's Creators"
    },
    {
      "code": 6011,
      "name": "InvaliedMetadata",
      "msg": "Invalid Metadata Address"
    }
  ]
};

export const IDL: NftStaking = {
  "version": "0.1.0",
  "name": "nft_staking",
  "instructions": [
    {
      "name": "initializeGlobal",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "globalName",
          "type": "string"
        },
        {
          "name": "nftCreator",
          "type": "publicKey"
        },
        {
          "name": "rewardTokenMint",
          "type": "publicKey"
        },
        {
          "name": "traitRates",
          "type": {
            "vec": "u64"
          }
        },
        {
          "name": "traitNames",
          "type": {
            "vec": "string"
          }
        },
        {
          "name": "normalRate",
          "type": "u64"
        },
        {
          "name": "lockDurations",
          "type": "bytes"
        },
        {
          "name": "lockRates",
          "type": {
            "vec": "u64"
          }
        },
        {
          "name": "custodial",
          "type": "bool"
        }
      ],
      "returns": null
    },
    {
      "name": "updateAdmin",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "newAdmin",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "updateGlobal",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "nftCreator",
          "type": "publicKey"
        },
        {
          "name": "rewardTokenMint",
          "type": "publicKey"
        },
        {
          "name": "traitRates",
          "type": {
            "vec": "u64"
          }
        },
        {
          "name": "traitNames",
          "type": {
            "vec": "string"
          }
        },
        {
          "name": "normalRate",
          "type": "u64"
        },
        {
          "name": "lockDurations",
          "type": "bytes"
        },
        {
          "name": "lockRates",
          "type": {
            "vec": "u64"
          }
        },
        {
          "name": "custodial",
          "type": "bool"
        }
      ],
      "returns": null
    },
    {
      "name": "initializeFixedPool",
      "accounts": [
        {
          "name": "userFixedPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [],
      "returns": null
    },
    {
      "name": "stakeNftToFixed",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userFixedPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultPda",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "edition",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mintMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "lockPeriod",
          "type": "u8"
        },
        {
          "name": "role",
          "type": "string"
        },
        {
          "name": "model",
          "type": "u64"
        }
      ],
      "returns": null
    },
    {
      "name": "withdrawNftFromFixed",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userFixedPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPda",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "edition",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "vaultStakeBump",
          "type": "u8"
        }
      ],
      "returns": null
    },
    {
      "name": "claimRewardAll",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userFixedPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRewardAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        }
      ],
      "returns": null
    },
    {
      "name": "claimReward",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userFixedPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRewardAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        }
      ],
      "returns": null
    },
    {
      "name": "withdrawReward",
      "accounts": [
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "globalAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimerRewardAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "globalBump",
          "type": "u8"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ],
      "returns": null
    }
  ],
  "accounts": [
    {
      "name": "globalPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "nftCreator",
            "type": "publicKey"
          },
          {
            "name": "rewardTokenMint",
            "type": "publicKey"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "traitRates",
            "type": {
              "vec": "u64"
            }
          },
          {
            "name": "traitNames",
            "type": {
              "vec": "string"
            }
          },
          {
            "name": "normalRate",
            "type": "u64"
          },
          {
            "name": "lockDurations",
            "type": "bytes"
          },
          {
            "name": "lockRates",
            "type": {
              "vec": "u64"
            }
          },
          {
            "name": "custodial",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "userPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "itemCount",
            "type": "u64"
          },
          {
            "name": "items",
            "type": {
              "array": [
                {
                  "defined": "StakedNFT"
                },
                150
              ]
            }
          },
          {
            "name": "rewardTime",
            "type": "i64"
          },
          {
            "name": "pendingReward",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "StakedNFT",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "nftAddr",
            "type": "publicKey"
          },
          {
            "name": "stakeTime",
            "type": "i64"
          },
          {
            "name": "rewardTime",
            "type": "i64"
          },
          {
            "name": "lockTime",
            "type": "i64"
          },
          {
            "name": "rate",
            "type": "i64"
          },
          {
            "name": "model",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidUserPool",
      "msg": "Invalid User Pool"
    },
    {
      "code": 6001,
      "name": "InvalidCollection",
      "msg": "Invalid Collection"
    },
    {
      "code": 6002,
      "name": "InvalidAdmin",
      "msg": "Invalid User Pool"
    },
    {
      "code": 6003,
      "name": "InvalidPoolError",
      "msg": "Invalid pool number"
    },
    {
      "code": 6004,
      "name": "InvalidNFTAddress",
      "msg": "No Matching NFT to withdraw"
    },
    {
      "code": 6005,
      "name": "InvalidOwner",
      "msg": "NFT Owner key mismatch"
    },
    {
      "code": 6006,
      "name": "InvalidWithdrawTime",
      "msg": "Staking Locked Now"
    },
    {
      "code": 6007,
      "name": "IndexOverflow",
      "msg": "Withdraw NFT Index OverFlow"
    },
    {
      "code": 6008,
      "name": "BeforeLockTime",
      "msg": "You can't Unstake Before LockTime"
    },
    {
      "code": 6009,
      "name": "LackLamports",
      "msg": "Insufficient Lamports"
    },
    {
      "code": 6010,
      "name": "MetadataCreatorParseError",
      "msg": "Can't Parse The NFT's Creators"
    },
    {
      "code": 6011,
      "name": "InvaliedMetadata",
      "msg": "Invalid Metadata Address"
    }
  ]
};
