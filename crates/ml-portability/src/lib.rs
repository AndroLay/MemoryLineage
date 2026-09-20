#![forbid(unsafe_code)]

use ml_local_evm::{
    LocalEvmError, POLKADOT_HUB_TESTNET_CHAIN_ID, run_demo_space_v2, run_demo_space_v2_on_chain,
};
use ml_spec_types::{
    EvidenceBundleV2, POLKADOT_HUB_TESTNET_CHAIN_ID as POLKADOT_HUB_TESTNET_CHAIN_ID_TEXT,
    PORTABILITY_REHEARSAL_V1, PortabilityComparison, PortabilityObservation,
    PortabilityRehearsalReport, PortabilityTarget,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PortabilityError {
    #[error("local EVM execution failed: {0}")]
    Local(#[from] LocalEvmError),
    #[error("portability invariant failed: {0}")]
    Invariant(&'static str),
}

fn transition_projection(bundle: &EvidenceBundleV2) -> Vec<(&str, &str)> {
    bundle
        .transitions
        .iter()
        .map(|transition| {
            (
                transition.transition_id.as_str(),
                transition.next_state_root.as_str(),
            )
        })
        .collect()
}

pub fn run_polkadot_hub_rehearsal(
    snapshot_commitments: &[String],
) -> Result<PortabilityRehearsalReport, PortabilityError> {
    let ethereum = run_demo_space_v2(snapshot_commitments)?;
    let polkadot = run_demo_space_v2_on_chain(POLKADOT_HUB_TESTNET_CHAIN_ID, snapshot_commitments)?;

    if transition_projection(&ethereum) != transition_projection(&polkadot) {
        return Err(PortabilityError::Invariant(
            "canonical transition projection changed across chain contexts",
        ));
    }
    if ethereum.authorization_history != polkadot.authorization_history {
        return Err(PortabilityError::Invariant(
            "authority history changed across chain contexts",
        ));
    }

    let ethereum_attack = ethereum.attack.as_ref().ok_or(PortabilityError::Invariant(
        "Ethereum rehearsal has no attack",
    ))?;
    let polkadot_attack = polkadot.attack.as_ref().ok_or(PortabilityError::Invariant(
        "Polkadot rehearsal has no attack",
    ))?;
    if ethereum_attack.reason != polkadot_attack.reason
        || ethereum_attack.reason.as_deref() != Some("BAD_PREVIOUS_STATE")
    {
        return Err(PortabilityError::Invariant(
            "stale predecessor result changed across chain contexts",
        ));
    }

    let ethereum_domain = ethereum
        .authorization_proofs
        .first()
        .ok_or(PortabilityError::Invariant(
            "Ethereum rehearsal has no authorization proof",
        ))?
        .domain_separator
        .clone();
    let polkadot_domain = polkadot
        .authorization_proofs
        .first()
        .ok_or(PortabilityError::Invariant(
            "Polkadot rehearsal has no authorization proof",
        ))?
        .domain_separator
        .clone();
    if ethereum_domain == polkadot_domain {
        return Err(PortabilityError::Invariant(
            "EIP-712 domain did not change with chain ID",
        ));
    }

    Ok(PortabilityRehearsalReport {
        schema_version: PORTABILITY_REHEARSAL_V1.to_owned(),
        report_type: "polkadot_hub_revm_portability_rehearsal".to_owned(),
        status: "LOCAL_REHEARSAL_PASS".to_owned(),
        target: PortabilityTarget {
            network: "Polkadot Hub TestNet".to_owned(),
            chain_id: POLKADOT_HUB_TESTNET_CHAIN_ID_TEXT.to_owned(),
            execution_backend: "REVM / Solidity bytecode".to_owned(),
            deployment: "NOT_PERFORMED".to_owned(),
            public_rpc_observation: "NOT_PERFORMED".to_owned(),
        },
        comparison: PortabilityComparison {
            canonical_transitions: "MATCH".to_owned(),
            state_roots: "MATCH".to_owned(),
            stale_predecessor_rejection: "BAD_PREVIOUS_STATE".to_owned(),
            authority_history: "MATCH".to_owned(),
            eip712_domain: "CHAIN_BOUND_DIFFERENT".to_owned(),
            solidity_artifact: "SAME_COMMITTED_CREATION_BYTECODE".to_owned(),
        },
        claims: vec![
            "The published Solidity bytecode preserves the tested lineage invariants under the Polkadot Hub TestNet chain context in local revm.".to_owned(),
            "This report does not claim a Polkadot deployment, public RPC observation, or cross-chain consensus proof.".to_owned(),
        ],
        observations: vec![
            PortabilityObservation {
                label: "Ethereum-local".to_owned(),
                execution_backend: "REVM / Solidity bytecode".to_owned(),
                evidence: ethereum,
                domain_separator: ethereum_domain,
            },
            PortabilityObservation {
                label: "Polkadot Hub TestNet chain context".to_owned(),
                execution_backend: "REVM / Solidity bytecode".to_owned(),
                evidence: polkadot,
                domain_separator: polkadot_domain,
            },
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polkadot_hub_rehearsal_reports_cross_chain_invariants_without_claiming_deployment() {
        let commitments = vec![
            "0x1111111111111111111111111111111111111111111111111111111111111111".to_owned(),
            "0x2222222222222222222222222222222222222222222222222222222222222222".to_owned(),
            "0x3333333333333333333333333333333333333333333333333333333333333333".to_owned(),
        ];
        let report = run_polkadot_hub_rehearsal(&commitments)
            .expect("the local portability rehearsal should execute");

        assert_eq!(report.status, "LOCAL_REHEARSAL_PASS");
        assert_eq!(report.target.chain_id, POLKADOT_HUB_TESTNET_CHAIN_ID_TEXT);
        assert_eq!(report.target.deployment, "NOT_PERFORMED");
        assert_eq!(report.target.public_rpc_observation, "NOT_PERFORMED");
        assert_eq!(report.comparison.canonical_transitions, "MATCH");
        assert_eq!(report.comparison.state_roots, "MATCH");
        assert_eq!(
            report.comparison.stale_predecessor_rejection,
            "BAD_PREVIOUS_STATE"
        );
        assert_eq!(report.comparison.eip712_domain, "CHAIN_BOUND_DIFFERENT");
    }
}
