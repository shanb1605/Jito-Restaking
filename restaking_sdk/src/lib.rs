use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum RestakingInstruction {
    /// Initializes the global configuration
    #[account(0, writable, name = "config")]
    #[account(1, writable, signer, name = "admin")]
    #[account(2, name = "vault_program")]
    #[account(3, name = "system_program")]
    InitializeConfig,

    /// Initializes the AVS
    #[account(0, writable, name = "config")]
    #[account(1, writable, name = "avs")]
    #[account(2, writable, name = "avs_operator_list")]
    #[account(3, writable, name = "avs_vault_list")]
    #[account(4, writable, name = "avs_slasher_list")]
    #[account(5, writable, signer, name = "admin")]
    #[account(6, signer, name = "base")]
    #[account(7, name = "system_program")]
    InitializeAvs,

    /// AVS adds support for receiving delegation from a vault
    #[account(0, name = "config")]
    #[account(1, writable, name = "avs")]
    #[account(2, writable, name = "avs_vault_list")]
    #[account(3, writable, signer, name = "admin")]
    #[account(4, name = "vault_program")]
    #[account(5, name = "vault")]
    #[account(6, name = "vault_config")]
    #[account(7, writable, name = "vault_avs_list")]
    #[account(8, writable, signer, name = "payer")]
    #[account(9, name = "system_program")]
    AvsAddVault,

    /// AVS removes support for receiving delegation from a vault
    #[account(0, name = "config")]
    #[account(1, writable, name = "avs")]
    #[account(2, writable, name = "avs_vault_list")]
    #[account(3, writable, signer, name = "admin")]
    #[account(4, name = "vault_program")]
    #[account(5, name = "vault")]
    #[account(6, name = "vault_config")]
    #[account(7, writable, name = "vault_avs_list")]
    #[account(8, writable, signer, name = "payer")]
    #[account(9, name = "system_program")]
    AvsRemoveVault,

    /// Initializes a operator
    #[account(0, writable, name = "config")]
    #[account(1, writable, name = "node_operator")]
    #[account(2, writable, name = "node_operator_avs_list")]
    #[account(3, writable, name = "node_operator_vault_list")]
    #[account(4, writable, signer, name = "admin")]
    #[account(5, signer, name = "base")]
    #[account(6, name = "system_program")]
    InitializeOperator,

    /// Sets the admin for a node operator
    #[account(0, writable, name = "node_operator")]
    #[account(1, signer, name = "old_admin")]
    #[account(2, signer, name = "new_admin")]
    OperatorSetAdmin,

    /// Sets the voter for a node operator
    #[account(0, writable, name = "node_operator")]
    #[account(1, signer, name = "admin")]
    #[account(2, name = "voter")]
    OperatorSetVoter,

    /// Node operator adds support for receiving delegation from a vault
    #[account(0, name = "config")]
    #[account(1, writable, name = "node_operator")]
    #[account(2, writable, name = "node_operator_vault_list")]
    #[account(3, writable, signer, name = "admin")]
    #[account(4, name = "vault_program")]
    #[account(5, name = "vault")]
    #[account(6, name = "vault_config")]
    #[account(7, writable, name = "vault_operator_list")]
    #[account(8, writable, signer, name = "payer")]
    #[account(9, name = "system_program")]
    OperatorAddVault,

    /// Node operator removes support for receiving delegation from a vault
    #[account(0, name = "config")]
    #[account(1, writable, name = "node_operator")]
    #[account(2, writable, name = "node_operator_vault_list")]
    #[account(3, writable, signer, name = "admin")]
    #[account(4, name = "vault_program")]
    #[account(5, name = "vault")]
    #[account(6, name = "vault_config")]
    #[account(7, writable, name = "vault_operator_list")]
    #[account(8, writable, signer, name = "payer")]
    #[account(9, name = "system_program")]
    OperatorRemoveVault,

    /// Node operator adds support for running an AVS
    #[account(0, name = "config")]
    #[account(1, name = "node_operator")]
    #[account(2, writable, name = "node_operator_avs_list")]
    #[account(3, name = "avs")]
    #[account(4, writable, signer, name = "admin")]
    OperatorAddAvs,

    /// Node operator removes support for running an AVS
    #[account(0, name = "config")]
    #[account(1, name = "node_operator")]
    #[account(2, writable, name = "node_operator_avs_list")]
    #[account(3, name = "avs")]
    #[account(4, writable, signer, name = "admin")]
    OperatorRemoveAvs,

    /// After the node operator has opted-in to the network, the AVS can choose to add it
    #[account(0, name = "config")]
    #[account(1, name = "avs")]
    #[account(2, writable, name = "avs_operator_list")]
    #[account(3, name = "node_operator")]
    #[account(4, writable, name = "node_operator_avs_list")]
    #[account(5, writable, signer, name = "admin")]
    AvsAddNodeOperator,

    /// The node operator can remove itself from the network
    #[account(0, name = "config")]
    #[account(1, name = "avs")]
    #[account(2, writable, name = "avs_operator_list")]
    #[account(3, name = "node_operator")]
    #[account(4, writable, signer, name = "admin")]
    AvsRemoveNodeOperator,

    /// The AVS adds support for a vault slasher
    #[account(0, name = "config")]
    #[account(1, name = "avs")]
    #[account(2, name = "avs_vault_list")]
    #[account(3, writable, name = "avs_slasher_list")]
    #[account(4, name = "vault")]
    #[account(5, name = "slasher")]
    #[account(6, writable, signer, name = "admin")]
    #[account(7, writable, signer, name = "payer")]
    #[account(8, name = "system_program")]
    AvsAddVaultSlasher(u64),

    /// AVS removes support for a slasher
    #[account(0, name = "config")]
    #[account(1, name = "avs")]
    #[account(2, name = "avs_vault_list")]
    #[account(3, writable, name = "avs_slasher_list")]
    #[account(4, name = "vault")]
    #[account(5, name = "slasher")]
    #[account(6, writable, signer, name = "admin")]
    #[account(7, writable, signer, name = "payer")]
    AvsDeprecateVaultSlasher,
}

pub fn initialize_config(
    program_id: &Pubkey,
    config: &Pubkey,
    admin: &Pubkey,
    vault_program: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*admin, true),
            AccountMeta::new_readonly(*vault_program, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: RestakingInstruction::InitializeConfig.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn initialize_avs(
    program_id: &Pubkey,
    config: &Pubkey,
    avs: &Pubkey,
    avs_operator_list: &Pubkey,
    avs_vault_list: &Pubkey,
    avs_slasher_list: &Pubkey,
    admin: &Pubkey,
    base: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*avs, false),
            AccountMeta::new(*avs_operator_list, false),
            AccountMeta::new(*avs_vault_list, false),
            AccountMeta::new(*avs_slasher_list, false),
            AccountMeta::new(*admin, true),
            AccountMeta::new(*base, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: RestakingInstruction::InitializeAvs.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn avs_add_vault(
    program_id: &Pubkey,
    config: &Pubkey,
    avs: &Pubkey,
    avs_vault_list: &Pubkey,
    admin: &Pubkey,
    vault_program: &Pubkey,
    vault: &Pubkey,
    vault_config: &Pubkey,
    vault_avs_list: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*avs, false),
            AccountMeta::new(*avs_vault_list, false),
            AccountMeta::new(*admin, true),
            AccountMeta::new_readonly(*vault_program, false),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new_readonly(*vault_config, false),
            AccountMeta::new(*vault_avs_list, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: RestakingInstruction::AvsAddVault.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn avs_remove_vault(
    program_id: &Pubkey,
    config: &Pubkey,
    avs: &Pubkey,
    avs_vault_list: &Pubkey,
    admin: &Pubkey,
    vault_program: &Pubkey,
    vault: &Pubkey,
    vault_config: &Pubkey,
    vault_avs_list: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*avs, false),
            AccountMeta::new(*avs_vault_list, false),
            AccountMeta::new(*admin, true),
            AccountMeta::new_readonly(*vault_program, false),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new_readonly(*vault_config, false),
            AccountMeta::new(*vault_avs_list, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: RestakingInstruction::AvsRemoveVault.try_to_vec().unwrap(),
    }
}

pub fn initialize_operator(
    program_id: &Pubkey,
    config: &Pubkey,
    node_operator: &Pubkey,
    node_operator_avs_list: &Pubkey,
    node_operator_vault_list: &Pubkey,
    admin: &Pubkey,
    base: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*node_operator, false),
            AccountMeta::new(*node_operator_avs_list, false),
            AccountMeta::new(*node_operator_vault_list, false),
            AccountMeta::new(*admin, true),
            AccountMeta::new_readonly(*base, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: RestakingInstruction::InitializeOperator
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn operator_add_vault(
    program_id: &Pubkey,
    config: &Pubkey,
    operator: &Pubkey,
    operator_vault_list: &Pubkey,
    admin: &Pubkey,
    vault_program: &Pubkey,
    vault: &Pubkey,
    vault_config: &Pubkey,
    vault_operator_list: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*operator, false),
            AccountMeta::new(*operator_vault_list, false),
            AccountMeta::new(*admin, true),
            AccountMeta::new_readonly(*vault_program, false),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new_readonly(*vault_config, false),
            AccountMeta::new(*vault_operator_list, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: RestakingInstruction::OperatorAddVault.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn operator_remove_vault(
    program_id: &Pubkey,
    config: &Pubkey,
    operator: &Pubkey,
    operator_vault_list: &Pubkey,
    admin: &Pubkey,
    vault_program: &Pubkey,
    vault: &Pubkey,
    vault_config: &Pubkey,
    vault_operator_list: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*operator, false),
            AccountMeta::new(*operator_vault_list, false),
            AccountMeta::new(*admin, true),
            AccountMeta::new_readonly(*vault_program, false),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new_readonly(*vault_config, false),
            AccountMeta::new(*vault_operator_list, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: RestakingInstruction::OperatorRemoveVault
            .try_to_vec()
            .unwrap(),
    }
}