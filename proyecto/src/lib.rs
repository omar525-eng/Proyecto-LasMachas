use anchor_lang::prelude::*;

declare_id!("GQvz3kmWK9wKu37C3vnnY36cEgPTrpaN5CyeVSUgemwJ");

#[program]
pub mod las_machas_cloud {
    use super::*;

    pub fn inicializar_inventario(ctx: Context<InicializarInventario>, nombre: String) -> Result<()> {
        let inventario = &mut ctx.accounts.inventario;
        inventario.owner = *ctx.accounts.owner.key;
        inventario.nombre = nombre;
        Ok(())
    }

    pub fn agregar_salsa(ctx: Context<AgregarSalsa>, nombre: String, nivel_picor: String, cantidad: u32) -> Result<()> {
        let inventario = &mut ctx.accounts.inventario;
        
        let nueva_salsa = Salsa {
            nombre,
            nivel_picor,
            cantidad,
        };
        
        inventario.salsas.push(nueva_salsa);
        Ok(())
    }

    pub fn ver_inventario(ctx: Context<VerInventario>) -> Result<()> {
        let inventario = &ctx.accounts.inventario;
        msg!("Inventario de: {}", inventario.nombre);
        for salsa in &inventario.salsas {
            msg!("Salsa: {}, Picor: {}, Stock: {}", salsa.nombre, salsa.nivel_picor, salsa.cantidad);
        }
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct InicializarInventario<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 50 + 4 + (10 * 100), 
        seeds = [b"inventario", owner.key().as_ref()],
        bump
    )]
    pub inventario: Account<'info, Inventario>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AgregarSalsa<'info> {
    #[account(mut, has_one = owner)]
    pub inventario: Account<'info, Inventario>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerInventario<'info> {
    pub inventario: Account<'info, Inventario>,
}

#[account]
pub struct Inventario {
    pub owner: Pubkey,
    pub nombre: String,
    pub salsas: Vec<Salsa>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Salsa {
    pub nombre: String,
    pub nivel_picor: String,
    pub cantidad: u32,
}
