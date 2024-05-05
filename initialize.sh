soroban contract invoke \
    --rpc-url "https://soroban-testnet.stellar.org:443" \
    --network-passphrase "Test SDF Network ; September 2015" \
    --network testnet \
    --id CC3QBUVTXRRIW3GOQ7JG2URJU2CCK4VDYAW7QGDGUJU6L5MSQMINZXIK \
    --source-account oggy \
    -- \
    initialize \
    --initial_members "[('GCCCK5QP24RVHAFFJNBPF6UBQ3RBLG3Y4U2UDNKKXZPQSNLVOC4JOUFS'),('GDOQ5IC2STP43BHTMTHUTDY3OUEAZUEY5IKRJUTXNVZUVIRPAPBWSRAY'),('GCENNJQ22BPJRE4UBFRMGOJGN2IMWZSUMSW76H2JGPQCS52WW7R3XOJS')]" \
    --metadata "{min_proposal_duration: 3600, max_proposal_duration: 604800, min_quorum_percentage: 50}"