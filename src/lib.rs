use anchor_lang::prelude::*;

declare_id!("5s3YhX3DyzgeMng669Hg17fEeH64iLCdSp4dn6uYW8Cf");

#[program]
pub mod joyeria_blockchain {
    use super::*;

    pub fn inicializar_inventario(
        context: Context<NuevoInventario>,
        nombre_sucursal: String,
    ) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let diamantes: Vec<Diamante> = Vec::new();

        context.accounts.inventario.set_inner(Inventario {
            owner: owner_id,
            nombre_sucursal,
            diamantes,
        });
        Ok(())
    }

    pub fn registrar_diamante(
        context: Context<ModificarInventario>,
        numero_serie: String,
        quilates: u16,
    ) -> Result<()> {
        require!(
            context.accounts.inventario.owner == context.accounts.owner.key(),
            Errores::NoAutorizado
        );

        let diamante = Diamante {
            numero_serie,
            quilates,
            autenticado: true,
        };

        context.accounts.inventario.diamantes.push(diamante);

        Ok(())
    }

    pub fn remover_diamante(
        context: Context<ModificarInventario>,
        numero_serie: String,
    ) -> Result<()> {
        require!(
            context.accounts.inventario.owner == context.accounts.owner.key(),
            Errores::NoAutorizado
        );

        let diamantes = &mut context.accounts.inventario.diamantes;

        for i in 0..diamantes.len() {
            if diamantes[i].numero_serie == numero_serie {
                diamantes.remove(i);
                msg!("Diamante {} removido del sistema!", numero_serie);
                return Ok(());
            }
        }
        Err(Errores::DiamanteNoEncontrado.into())
    }

    pub fn ver_inventario(context: Context<ModificarInventario>) -> Result<()> {
        require!(
            context.accounts.inventario.owner == context.accounts.owner.key(),
            Errores::NoAutorizado
        );

        msg!(
            "Lista de diamantes en stock: {:#?}",
            context.accounts.inventario.diamantes
        );
        Ok(())
    }

    pub fn alternar_autenticacion(
        context: Context<ModificarInventario>,
        numero_serie: String,
    ) -> Result<()> {
        require!(
            context.accounts.inventario.owner == context.accounts.owner.key(),
            Errores::NoAutorizado
        );

        let diamantes = &mut context.accounts.inventario.diamantes;
        for i in 0..diamantes.len() {
            let estado = diamantes[i].autenticado;

            if diamantes[i].numero_serie == numero_serie {
                let nuevo_estado = !estado;
                diamantes[i].autenticado = nuevo_estado;
                msg!(
                    "Diamante: {} - Estado autenticado: {}",
                    numero_serie,
                    nuevo_estado
                );
                return Ok(());
            }
        }

        Err(Errores::DiamanteNoEncontrado.into())
    }

    pub fn transferir_diamante(
        context: Context<ModificarInventario>,
        numero_serie: String,
    ) -> Result<()> {
        require!(
            context.accounts.inventario.owner == context.accounts.owner.key(),
            Errores::NoAutorizado
        );

        let diamantes = &mut context.accounts.inventario.diamantes;

        for i in 0..diamantes.len() {
            if diamantes[i].numero_serie == numero_serie {
                diamantes.remove(i);
                msg!("Diamante {} transferido exitosamente", numero_serie);
                return Ok(());
            }
        }
        Err(Errores::DiamanteNoEncontrado.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no tienes permisos sobre este inventario")]
    NoAutorizado,
    #[msg("Error, el diamante solicitado no existe en el registro")]
    DiamanteNoEncontrado,
}

#[account]
#[derive(InitSpace)]
pub struct Inventario {
    owner: Pubkey,

    #[max_len(50)]
    nombre_sucursal: String,

    #[max_len(20)]
    diamantes: Vec<Diamante>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Diamante {
    #[max_len(32)]
    numero_serie: String,
    quilates: u16,
    autenticado: bool,
}

#[derive(Accounts)]
pub struct NuevoInventario<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
	init,
	payer = owner,
	space = Inventario::INIT_SPACE + 8,
	seeds = [b"joyeria", owner.key().as_ref()],
	bump
	)]
    pub inventario: Account<'info, Inventario>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarInventario<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub inventario: Account<'info, Inventario>,
}
