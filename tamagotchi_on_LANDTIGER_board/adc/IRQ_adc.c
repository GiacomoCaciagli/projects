/*********************************************************************************************************
**--------------File Info---------------------------------------------------------------------------------
** File name:           IRQ_adc.c
** Last modified Date:  20184-12-30
** Last Version:        V1.00
** Descriptions:        functions to manage A/D interrupts
** Correlated files:    adc.h
**--------------------------------------------------------------------------------------------------------       
*********************************************************************************************************/

#include "lpc17xx.h"
#include "adc.h"
#include "../timer/timer.h"
#include "../GLCD/GLCD.h"

/*----------------------------------------------------------------------------
  A/D IRQ: Executed when A/D Conversion is ready (signal from ADC peripheral)
 *----------------------------------------------------------------------------*/

unsigned short AD_current;   
unsigned short AD_last = 0xFF;     /* Last converted value               */

void ADC_IRQHandler(void) {
  static int last_volume=0;
  AD_current = ((LPC_ADC->ADGDR>>4) & 0xFFF);/* Read Conversion Result             */
  
	//init_timer(3,freq[AD_current*7/0xFFF]);
	volume=(10*AD_current)/4096;
	if(last_volume!=volume)
	{
		switch(volume){
			case 0:
				Draw_volume(0);
				break;
			case 1:
			case 2:
			case 3:
				Draw_volume(1);
				break;
			case 4:
			case 5:
			case 6:
				Draw_volume(2);
				break;
			default:
				Draw_volume(3);
				break;
		}
		last_volume=volume;
	}
}
