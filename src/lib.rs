use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("");

#[program]
pub mod diamond_cert {
    use super::*;

    pub fn registrar_joyeria(
        ctx: Context<RegistrarJoyeria>,
        nombre: String,
        pais: String,
        licencia_id: String,
    ) -> Result<()> {
        require!(nombre.len() <= 60, ErrorCert::NombreMuyLargo);
        require!(licencia_id.len() <= 40, ErrorCert::LicenciaInvalida);

        let joyeria = &mut ctx.accounts.joyeria;
        joyeria.autoridad = ctx.accounts.autoridad.key();
        joyeria.nombre = nombre;
        joyeria.pais = pais;
        joyeria.licencia_id = licencia_id;
        joyeria.total_emitidos = 0;
        joyeria.activa = true;
        joyeria.bump = ctx.bumps.joyeria;

        emit!(JoyeliaRegistrada {
            autoridad: joyeria.autoridad,
            nombre: joyeria.nombre.clone(),
        });
        Ok(())
    }

    pub fn emitir_certificado(
        ctx: Context<EmitirCertificado>,
        params: ParametrosCertificado,
    ) -> Result<()> {
        require!(ctx.accounts.joyeria.activa, ErrorCert::JoyeliaInactiva);
        require!(params.numero_serie.len() <= 32, ErrorCert::SerieInvalida);

        let cert = &mut ctx.accounts.certificado;
        cert.numero_serie = params.numero_serie.clone();
        cert.joyeria = ctx.accounts.joyeria.key();
        cert.propietario = ctx.accounts.comprador.key();
        cert.mint = ctx.accounts.mint.key();
        cert.quilates = params.quilates;
        cert.corte = params.corte;
        cert.color = params.color;
        cert.claridad = params.claridad;
        cert.precio_usd = params.precio_usd;
        cert.fecha_emision = Clock::get()?.unix_timestamp;
        cert.ultima_transf = Clock::get()?.unix_timestamp;
        cert.num_transferencias = 0;
        cert.estado = EstadoCert::Activo;
        cert.reportado_robado = false;
        cert.bump = ctx.bumps.certificado;

        let entrada = EntradaHistorial {
            timestamp: cert.fecha_emision,
            de: ctx.accounts.joyeria.autoridad,
            para: ctx.accounts.comprador.key(),
            tipo_evento: TipoEvento::Emision,
            precio_usd: cert.precio_usd,
        };
        cert.historial.push(entrada);

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account_comprador.to_account_info(),
                    authority: ctx.accounts.joyeria_auth_pda.to_account_info(),
                },
                &[&[
                    b"joyeria_auth",
                    ctx.accounts.joyeria.key().as_ref(),
                    &[ctx.bumps.joyeria_auth_pda],
                ]],
            ),
            1,
        )?;

        ctx.accounts.joyeria.total_emitidos += 1;
        Ok(())
    }

    pub fn transferir_certificado(
        ctx: Context<TransferirCertificado>,
        precio_venta_usd: u64,
    ) -> Result<()> {
        let cert = &mut ctx.accounts.certificado;
        let vendedor_anterior = cert.propietario;
        let nuevo_propietario = ctx.accounts.comprador.key();
        let ts_ahora = Clock::get()?.unix_timestamp;

        cert.propietario = nuevo_propietario;
        cert.ultima_transf = ts_ahora;
        cert.num_transferencias += 1;
        cert.precio_usd = precio_venta_usd;

        cert.historial.push(EntradaHistorial {
            timestamp: ts_ahora,
            de: vendedor_anterior,
            para: nuevo_propietario,
            tipo_evento: TipoEvento::Transferencia,
            precio_usd: precio_venta_usd,
        });

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_vendedor.to_account_info(),
                    to: ctx.accounts.token_comprador.to_account_info(),
                    authority: ctx.accounts.vendedor.to_account_info(),
                },
            ),
            1,
        )?;
        Ok(())
    }

    pub fn reportar_robo(ctx: Context<ReportarRobo>, _numero_policia: String) -> Result<()> {
        let cert = &mut ctx.accounts.certificado;
        let reportante = ctx.accounts.reportante.key();
        let ts_ahora = Clock::get()?.unix_timestamp;

        cert.reportado_robado = true;
        cert.estado = EstadoCert::Robado;

        cert.historial.push(EntradaHistorial {
            timestamp: ts_ahora,
            de: reportante,
            para: reportante,
            tipo_evento: TipoEvento::ReporteRobo,
            precio_usd: 0,
        });
        Ok(())
    }

    pub fn actualizar_tasacion(
        ctx: Context<ActualizarTasacion>,
        nuevo_precio_usd: u64,
        _notas: String,
    ) -> Result<()> {
        let cert = &mut ctx.accounts.certificado;
        let autoridad_joyeria = ctx.accounts.joyeria.autoridad;
        let propietario_actual = cert.propietario;
        let ts_ahora = Clock::get()?.unix_timestamp;

        cert.precio_usd = nuevo_precio_usd;
        cert.historial.push(EntradaHistorial {
            timestamp: ts_ahora,
            de: autoridad_joyeria,
            para: propietario_actual,
            tipo_evento: TipoEvento::Retasacion,
            precio_usd: nuevo_precio_usd,
        });
        Ok(())
    }

    pub fn revocar_certificado(ctx: Context<RevocarCertificado>, _motivo: String) -> Result<()> {
        let cert = &mut ctx.accounts.certificado;
        let autoridad_joyeria = ctx.accounts.joyeria.autoridad;
        let propietario_actual = cert.propietario;
        let ts_ahora = Clock::get()?.unix_timestamp;

        cert.estado = EstadoCert::Revocado;
        cert.historial.push(EntradaHistorial {
            timestamp: ts_ahora,
            de: autoridad_joyeria,
            para: propietario_actual,
            tipo_evento: TipoEvento::Revocacion,
            precio_usd: 0,
        });
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct RegistrarJoyeria<'info> {
    #[account(mut)]
    pub autoridad: Signer<'info>,
    #[account(
        init, payer = autoridad, space = Joyeria::SPACE,
        seeds = [b"joyeria", autoridad.key().as_ref()], bump
    )]
    pub joyeria: Account<'info, Joyeria>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(params: ParametrosCertificado)]
pub struct EmitirCertificado<'info> {
    #[account(mut)]
    pub autoridad: Signer<'info>,
    #[account(
        mut, seeds = [b"joyeria", autoridad.key().as_ref()],
        bump = joyeria.bump, has_one = autoridad
    )]
    pub joyeria: Account<'info, Joyeria>,
    #[account(seeds = [b"joyeria_auth", joyeria.key().as_ref()], bump)]
    pub joyeria_auth_pda: UncheckedAccount<'info>,
    #[account(
        init, payer = autoridad, space = Certificado::SPACE,
        seeds = [b"certificado", params.numero_serie.as_bytes()], bump
    )]
    pub certificado: Account<'info, Certificado>,
    #[account(
        init, payer = autoridad, mint::decimals = 0,
        mint::authority = joyeria_auth_pda,
        seeds = [b"mint", params.numero_serie.as_bytes()], bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed, payer = autoridad,
        associated_token::mint = mint,
        associated_token::authority = comprador,
    )]
    pub token_account_comprador: Account<'info, TokenAccount>,
    pub comprador: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct TransferirCertificado<'info> {
    #[account(mut)]
    pub vendedor: Signer<'info>,
    pub comprador: UncheckedAccount<'info>,
    #[account(
        mut, seeds = [b"certificado", certificado.numero_serie.as_bytes()],
        bump = certificado.bump, has_one = mint
    )]
    pub certificado: Account<'info, Certificado>,
    pub mint: Account<'info, Mint>,
    #[account(mut, associated_token::mint = mint, associated_token::authority = vendedor)]
    pub token_vendedor: Account<'info, TokenAccount>,
    #[account(
        init_if_needed, payer = vendedor,
        associated_token::mint = mint,
        associated_token::authority = comprador
    )]
    pub token_comprador: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReportarRobo<'info> {
    #[account(mut)]
    pub reportante: Signer<'info>,
    #[account(mut)]
    pub certificado: Account<'info, Certificado>,
}

#[derive(Accounts)]
pub struct ActualizarTasacion<'info> {
    pub autoridad: Signer<'info>,
    #[account(seeds = [b"joyeria", autoridad.key().as_ref()], bump = joyeria.bump, has_one = autoridad)]
    pub joyeria: Account<'info, Joyeria>,
    #[account(mut)]
    pub certificado: Account<'info, Certificado>,
}

#[derive(Accounts)]
pub struct RevocarCertificado<'info> {
    pub autoridad: Signer<'info>,
    #[account(seeds = [b"joyeria", autoridad.key().as_ref()], bump = joyeria.bump, has_one = autoridad)]
    pub joyeria: Account<'info, Joyeria>,
    #[account(mut)]
    pub certificado: Account<'info, Certificado>,
}

#[account]
pub struct Joyeria {
    pub autoridad: Pubkey,
    pub nombre: String,
    pub pais: String,
    pub licencia_id: String,
    pub total_emitidos: u32,
    pub activa: bool,
    pub bump: u8,
}
impl Joyeria {
    pub const SPACE: usize = 8 + 32 + 64 + 32 + 40 + 4 + 1 + 1;
}

#[account]
pub struct Certificado {
    pub numero_serie: String,
    pub joyeria: Pubkey,
    pub propietario: Pubkey,
    pub mint: Pubkey,
    pub quilates: u16,
    pub corte: String,
    pub color: String,
    pub claridad: String,
    pub precio_usd: u64,
    pub fecha_emision: i64,
    pub ultima_transf: i64,
    pub num_transferencias: u16,
    pub estado: EstadoCert,
    pub reportado_robado: bool,
    pub historial: Vec<EntradaHistorial>,
    pub bump: u8,
}
impl Certificado {
    pub const SPACE: usize =
        8 + 36 + 32 + 32 + 32 + 2 + 20 + 10 + 10 + 8 + 8 + 8 + 2 + 1 + 1 + (4 + 10 * 80) + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum EstadoCert {
    Activo,
    Revocado,
    Robado,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct EntradaHistorial {
    pub timestamp: i64,
    pub de: Pubkey,
    pub para: Pubkey,
    pub tipo_evento: TipoEvento,
    pub precio_usd: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum TipoEvento {
    Emision,
    Transferencia,
    ReporteRobo,
    Retasacion,
    Revocacion,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ParametrosCertificado {
    pub numero_serie: String,
    pub quilates: u16,
    pub corte: String,
    pub color: String,
    pub claridad: String,
    pub precio_usd: u64,
}

#[event]
pub struct JoyeliaRegistrada {
    pub autoridad: Pubkey,
    pub nombre: String,
}

#[error_code]
pub enum ErrorCert {
    #[msg("La joyería no está activa")]
    JoyeliaInactiva,
    #[msg("Nombre muy largo")]
    NombreMuyLargo,
    #[msg("Licencia inválida")]
    LicenciaInvalida,
    #[msg("Serie inválida")]
    SerieInvalida,
}
