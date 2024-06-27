/*********************************************************************************************************
**--------------File Info---------------------------------------------------------------------------------
** File name:           IRQ_RIT.c
** Last modified Date:  2014-09-25
** Last Version:        V1.00
** Descriptions:        functions to manage T0 and T1 interrupts
** Correlated files:    RIT.h
**--------------------------------------------------------------------------------------------------------
*********************************************************************************************************/
#include "lpc17xx.h"
#include "RIT.h"
#include "../GLCD/GLCD.h"
#include "../timer/timer.h"
#include "../adc/adc.h"

/******************************************************************************
** Function name:		RIT_IRQHandler
**
** Descriptions:		REPETITIVE INTERRUPT TIMER handler
**
** parameters:			None
** Returned value:		None
**
******************************************************************************/

volatile int down=0;

/*
262Hz	k=2120		c4
294Hz	k=1890		
330Hz	k=1684		
349Hz	k=1592		
392Hz	k=1417		
440Hz	k=1263		
494Hz	k=1125		
523Hz	k=1062		c5

*/

void RIT_IRQHandler (void)
{				
	if((LPC_GPIO1->FIOPIN & (1<<28)) == 0&&select!=3&&action==0){	//right
		select=1;
		disable_timer(3);
		reset_timer(3);
		init_timer(3,freqs[0]);
		enable_timer(3);
		LCD_DrawRect(0,249,119,3,Blue);
		LCD_DrawRect(0,249,3,91,Blue);
		LCD_DrawRect(0,317,119,3,Blue);
		LCD_DrawRect(119,249,121,3,Red);
		LCD_DrawRect(119,249,3,91,Red);
		LCD_DrawRect(237,249,3,91,Red);
		LCD_DrawRect(119,317,121,3,Red);
	}
	
	/* button management */
	if((LPC_GPIO1->FIOPIN & (1<<27)) == 0&&select!=3&&action==0){	//left
		select=2;
		disable_timer(3);
		reset_timer(3);
		init_timer(3,freqs[0]);
		enable_timer(3);
		LCD_DrawRect(121,249,119,3,Blue);
		LCD_DrawRect(237,249,3,91,Blue);
		LCD_DrawRect(121,317,119,3,Blue);
		LCD_DrawRect(0,249,121,3,Red);
		LCD_DrawRect(0,249,3,91,Red);
		LCD_DrawRect(119,249,3,91,Red);
		LCD_DrawRect(0,317,121,3,Red);
	}
	
	if((LPC_GPIO1->FIOPIN & (1<<25)) == 0 &&select!=0&&action==0){
		disable_timer(3);
		reset_timer(3);
		init_timer(3,freqs[0]);
		enable_timer(3);
		disable_RIT();
		
		disable_timer(1);
		if(select!=3)
		{
			disable_timer(0);
			action=1;
			if(select==1)
			{
				Draw_phantom(190,160);
			}else
			{
				Draw_ball(30,160);
			}
			enable_timer(0);
			enable_timer(2);
			disable_timer(3);
			disable_timer(3);
			reset_timer(3);
			init_timer(3,freqs[4]);
			enable_timer(3);
			
		}else if(select==3)
		{
			disable_timer(0);
			select=0;
			happy=3;
			sat=3;
			LCD_start(Blue,Black);
			reset=1;
			reset_timer(0);
			reset_timer(1);
			reset_RIT();
			enable_timer(0);
			enable_timer(1);
			enable_RIT();
		}
	}
	
	ADC_start_conversion();	
  LPC_RIT->RICTRL |= 0x1;	/* clear interrupt flag */
  return;
}

/******************************************************************************
**                            End Of File
******************************************************************************/
