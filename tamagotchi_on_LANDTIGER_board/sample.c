/****************************************Copyright (c)****************************************************
**                                      
**                                 http://www.powermcu.com
**
**--------------File Info---------------------------------------------------------------------------------
** File name:               main.c
** Descriptions:            The GLCD application function
**
**--------------------------------------------------------------------------------------------------------
** Created by:              AVRman
** Created date:            2010-11-7
** Version:                 v1.0
** Descriptions:            The original version
**
**--------------------------------------------------------------------------------------------------------
** Modified by:             Paolo Bernardi
** Modified date:           03/01/2020
** Version:                 v2.0
** Descriptions:            basic program for LCD and Touch Panel teaching
**
*********************************************************************************************************/

/* Includes ------------------------------------------------------------------*/
#include "LPC17xx.h"
#include "GLCD/GLCD.h" 
#include "TouchPanel/TouchPanel.h"
#include "timer/timer.h"
#include "RIT/RIT.h"
#include "adc/adc.h"

#define SIMULATOR 1

#ifdef SIMULATOR
extern uint8_t ScaleFlag; // <- ScaleFlag needs to visible in order for the emulator to find the symbol (can be placed also inside system_LPC17xx.h but since it is RO, it needs more work)
#endif


int main(void)
{
  SystemInit();  												/* System Initialization (i.e., PLL)  */
	
	ADC_init();
  LCD_Initialization();
	  
	TP_Init();
	TouchPanel_Calibrate();

	LCD_start(Blue,Black);

	init_timer(0,0x017D7840);
	enable_timer(0);//età e vita
	NVIC_SetPriority(TIMER0_IRQn,2);
	init_timer(1,0x000000C8);
	enable_timer(1);//touch
	NVIC_SetPriority(TIMER1_IRQn,2);
	init_timer(2,0x00BEBC20);//animazioni
	NVIC_SetPriority(TIMER2_IRQn,2);
	init_timer(3,0x00001000);//suono
	NVIC_SetPriority(TIMER3_IRQn,0);
	init_RIT(0x004C4B40);
	enable_RIT();
	NVIC_SetPriority(RIT_IRQn,2);
	
	LPC_SC->PCON |= 0x1;									/* power-down	mode										*/
	LPC_SC->PCON &= ~(0x2);	

	LPC_PINCON->PINSEL1 |= (1<<21);
	LPC_PINCON->PINSEL1 &= ~(1<<20);
	LPC_GPIO0->FIODIR |= (1<<26);
	
  while (1)	
  {
		__ASM("wfi");
  }
}

/*********************************************************************************************************
      END FILE
*********************************************************************************************************/
