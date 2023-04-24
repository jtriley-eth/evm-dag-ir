# EVM Directed Acyclic Graph Intermediate Representation

solc:

```solc
contract a {
    fallback() external {
        uint256 x = calldataload(0);
        return x + 1;
    }
}
```

huff:

```huff
#define macro MAIN() = takes (0) returns (0) {
    push0
    calldataload

    push 0x01
    add

    push0
    mstore

    push 0x20
    push0
    return
}
```

dag ir:

```
flowchart RL:
    0 --> calldataload
    res_calldataload --> v0

    v0 --> add
    1 --> add
    add_res --> v1

    v1 --> mstore
    0 --> mstore

    32 --> return
    0 --> return
```

```graph
graph {
    nodes [
        calldataload: {
            edges [
                data.imm0
            ],
        },
        add {
            edges [
                data.imm1
                calldataload
            ]
        }
        mstore {
            edges [
                add
                data.imm0
            ]
        }
        return {
            edges [
                data.imm0
                data.imm2
            ]
        }
    ],
}

data {
    imm0 : 0
    imm1 : 1
    imm2 : 32
}
```

```graph
node: {
    instruction: return,
    dependencies: [
        imm0,
        imm2,
    ],
}

node: {
    instruction: mstore
    dependencies: [
        node: {
            instruction: add
            dependencies: [
                node: {
                    instruction: calldataload,
                    dependencies: [
                        imm0,
                    ],
                },
            ],
        },
        imm0,
    ],
}
```
