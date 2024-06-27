/*********************************************************************************************************
**--------------File Info---------------------------------------------------------------------------------
** File name:           IRQ_timer.c
** Last modified Date:  2014-09-25
** Last Version:        V1.00
** Descriptions:        functions to manage T0 and T1 interrupts
** Correlated files:    timer.h
**--------------------------------------------------------------------------------------------------------
*********************************************************************************************************/
#include <string.h>
#include "lpc17xx.h"
#include "timer.h"
#include "../GLCD/GLCD.h" 
#include "../TouchPanel/TouchPanel.h"
#include "../RIT/RIT.h"

/******************************************************************************
** Function name:		Timer0_IRQHandler
**
** Descriptions:		Timer/Counter 0 interrupt handler
**
** parameters:			None
** Returned value:		None
**
******************************************************************************/

uint8_t happy_pos[]={47,53,59,65};
int8_t happy=3;
uint8_t sat_pos[]={167,173,179,185};
int8_t sat=3;
uint8_t reset=0;
uint8_t select=0;
uint8_t volume=0;
const uint16_t freqs[8]={2120,1890,1684,1592,1417,1263,1125,1062};
uint8_t action=0;//1=eating,2=cuddle,3=end


uint16_t SinTable[45]={410, 467, 523, 576, 627, 673, 714, 749, 778,799, 813, 819, 817, 807, 789, 764, 732, 694, 650, 602, 550, 495, 438, 381, 324, 270, 217,
												169, 125, 87 , 55 , 30 , 12 , 2  , 0  , 6  ,20 , 41 , 70 , 105, 146, 193, 243, 297, 353};


void TIMER0_IRQHandler (void) //vita e età
{
	int x_time[]={104,112,128,136,152,160};
	static uint8_t h1='0',h2='0',m1='0',m2='0',s1='0',s2='0';
	static int pos=1;
	if(reset==1)
	{
		h1=h2=m1=m2=s1=s2='0';
		reset=0;
	}
	s2++;
	if(s2>'9')
	{
		s2='0';
		s1++;
		if(s1>'5')
		{
			s2='0';
			m2++;
			if(m2>'9')
			{
				m2='0';
				m1++;
				if(m1>'6')
				{
					m1='0';
					h2++;
					if(h2>'9')
					{
						h2=0;
						h1++;
						GUI_Text(x_time[0],10,&h1,Blue,Black,0);
					}
					GUI_Text(x_time[1],10,&h2,Blue,Black,0);
				}
				GUI_Text(x_time[2],10,&m1,Blue,Black,0);
			}
			GUI_Text(x_time[3],10,&m2,Blue,Black,0);
		}
		GUI_Text(x_time[4],10,&s1,Blue,Black,0);
	}
	GUI_Text(x_time[5],10,&s2,Blue,Black,0);
	
	if(action==0&&reset==0)
	{
		if(pos%2==0)
		{
			mounth(1);
		}else
		{
			mounth(0);
		}
		if(pos%5==0&&pos!=0)
		{
			LCD_DrawRect(happy_pos[happy],53,5,10,Black);
			LCD_DrawRect(sat_pos[sat],53,5,10,Black);
			happy--;
			sat--;
			
			if(happy<=-1||sat<=-1)
			{
				disable_timer(1);
				disable_timer(0);
				disable_RIT();
				select=3;
				action=3;
				enable_timer(2);
				disable_timer(3);
				reset_timer(3);
				init_timer(3,freqs[2]);
				enable_timer(3);
			}
		}
			
		pos++;
	}
	
  LPC_TIM0->IR = 1;			/* clear interrupt flag */
  return;
}


/******************************************************************************
** Function name:		Timer1_IRQHandler
**
** Descriptions:		Timer/Counter 1 interrupt handler
**
** parameters:			None
** Returned value:		None
**
******************************************************************************/
void TIMER1_IRQHandler (void) //cuddling
{
	if(getDisplayPoint(&display, Read_Ads7846(), &matrix )){
		if(display.x>=60&&display.x<=180&&display.y>=100&&display.y<=220){
			action=2;
			disable_timer(0);
			disable_timer(1);
			Draw_pacman(120,160,Yellow,0);
			LCD_DrawRect(119,120,18,18,Yellow);
			LCD_DrawRect(89,124,2,5,Black);
			LCD_DrawRect(91,122,2,3,Black);
			LCD_DrawRect(93,120,10,2,Black);
			LCD_DrawRect(103,122,2,3,Black);
			LCD_DrawRect(105,124,2,5,Black);
			LCD_DrawRect(133,124,2,5,Black);
			LCD_DrawRect(135,122,2,3,Black);
			LCD_DrawRect(137,120,10,2,Black);
			LCD_DrawRect(147,122,2,3,Black);
			LCD_DrawRect(149,124,2,5,Black);
			enable_timer(0);
			enable_timer(2);
			disable_timer(3);
			reset_timer(3);
			init_timer(3,freqs[0]);
			enable_timer(3);
			
			//devo mettere l'animazione
		}
	}
	
  LPC_TIM1->IR = 1;			/* clear interrupt flag */
  return;
}

void TIMER2_IRQHandler(void) //temporizzazione animazioni
	{
	static int phase=0;
	int8_t pos[]={0,1,0,1,0,1,0,1,0,0,0,0,-1,-1};
	int x0,y0,x1;
	volatile int i;
	
	if(action==1)
	{
		switch(phase){
			case 0:
				disable_timer(0);
				Draw_pacman(120,160,Black,0);
				Draw_pacman(70+((select-2)*-100),160,Yellow,select+1-(select%2));
				enable_timer(0);
				break;
			case 1:
				disable_timer(0);
				Draw_pacman(70+((select-2)*-100),160,Yellow,select-(select%2));
				enable_timer(0);
			case 2:
				disable_timer(0);
				if(select==1)
				{
					if(happy<3)
					{
						happy++;
					}
					LCD_DrawRect(happy_pos[happy],53,5,10,Green);
				}else
				{
					if(sat<3)
					{
						sat++;
					}
					LCD_DrawRect(sat_pos[sat],53,5,10,Green);
				}
				Draw_pacman(70+((select-2)*-100),160,Black,0);
				Draw_pacman(120,160,Yellow,0);
				enable_timer(0);
				phase=-1;
				reset_RIT();
				enable_RIT();
				action=0;
				enable_timer(0);
				enable_timer(1);
				disable_timer(2);
				reset_timer(2);
				break;
		}
		phase++;
	}
	if(action==3)
	{
		switch(phase){
			case 0:
				Draw_pacman(120,160,Yellow,4);
				x0=95;
				y0=115;
				
				for(i=0;i<20;i++)
				{
					x0--;
					y0++;
					LCD_DrawLine(x0,y0,x0+13,y0,Yellow);
				}
				x0=134;
				y0=115;
				for(i=0;i<20;i++)
				{
					x0++;
					y0++;
					LCD_DrawLine(x0,y0,x0+13,y0,Yellow);
				}
				x0=95;
				y0=132;
				x1=x0;
				for(i=0;i<15;i++)
				{
					x0=x0-pos[i];
					y0++;
					x1=x1+pos[i];
					LCD_DrawLine(x0,y0,x1,y0,0x8FFF);
				}
				break;
			case 2:
				Draw_pacman(120,160,Black,0);
				Draw_pacman(120,160,Yellow,0);
				x0=125;
				y0=115;
				
				for(i=0;i<20;i++)
				{
					x0--;
					y0++;
					LCD_DrawLine(x0,y0,x0+10,y0,Yellow);
				}
				x0=133;
				y0=134;
				x1=x0;
				for(i=0;i<15;i++)
				{
					x0=x0-pos[i];
					y0++;
					x1=x1+pos[i];
					LCD_DrawLine(x0,y0,x1,y0,0x8FFF);
				}
				break;
			case 3:
				Draw_pacman(120,160,Black,0);
				Draw_pacman(180,160,Yellow,0);
				x0=185;
				y0=115;
				
				for(i=0;i<20;i++)
				{
					x0--;
					y0++;
					LCD_DrawLine(x0,y0,x0+10,y0,Yellow);
				}
				x0=193;
				y0=134;
				x1=x0;
				for(i=0;i<15;i++)
				{
					x0=x0-pos[i];
					y0++;
					x1=x1+pos[i];
					LCD_DrawLine(x0,y0,x1,y0,0x8FFF);
				}
				break;
			case 4:
				Draw_pacman(180,160,Black,0);
				Draw_pacman(240,160,Yellow,0);
				break;
			case 5:
				Draw_pacman(240,160,Black,0);
				LCD_DrawRect(0,249,240,71,Black);
				GUI_Text(56,91,(uint8_t*) "GAME",Blue,Black,2);
				GUI_Text(56,155,(uint8_t*) "OVER",Blue,Black,2);
				LCD_DrawRect(0,249,240,3,Red);
				LCD_DrawRect(0,249,3,71,Red);
				LCD_DrawRect(237,249,3,71,Red);
				LCD_DrawRect(0,317,240,3,Red);
				GUI_Text(64,264,(uint8_t*) "Restart",Blue,Black,1);
				reset_RIT();
				enable_RIT();
				disable_timer(2);
				reset_timer(2);
				phase=-1;
				action=0;
				break;
		}
		phase++;
	}
	if(action==2)
	{
		switch(phase){
			case 0:
				Draw_heart(50,230,Red);
				break;
			case 1:
				Draw_heart(200,130,Red);
				break;
			case 2:
				Draw_heart(210,220,Red);
				break;
			case 3:
				Draw_heart(40,120,Red);
				break;
			case 4:
				if(happy<3)
				{
					happy++;
					LCD_DrawRect(happy_pos[happy],53,5,10,Green);
				}
				disable_timer(0);
				Draw_heart(50,230,Black);
				Draw_heart(200,130,Black);
				Draw_heart(210,220,Black);
				Draw_heart(40,120,Black);
				Draw_pacman(120,160,Yellow,0);
				enable_timer(0);
				enable_timer(1);
				disable_timer(2);
				reset_timer(2);
				action=0;
				phase=-1;
				break;
		}
		phase++;
	}
  LPC_TIM2->IR = 1;			/* clear interrupt flag */
}

void TIMER3_IRQHandler(void) //suono
{
	static int ticks=0;
	static int tempo=0;
	static int note=0;
	static int max;
	/* DAC management */	
	LPC_DAC->DACR = (((SinTable[ticks]*1024/819)*volume/10)<<6)/2;//con 4096 al posto di 1024 funziona
	if(note==0)
	{
		switch(action){
			case 0:
					max=1000;
			break;
			case 1:
				max=7851;
			break;
			case 2:
				max=2948;
				break;
			case 3:
				max=5896;
				break;
			default:
				break;
		}
		enable_timer(3);
	}
	if(ticks==45)
	{
		ticks=0;
	}
	ticks++;
	tempo++;
	if(tempo>=max)
	{
		disable_timer(3);
		reset_timer(3);
		switch(action){
			case 0:
				note=0;
			break;
			case 1:
				switch(note){
					case 0:
						init_timer(3,freqs[5]);
						enable_timer(3);
						max=9897;
							note++;

					break;
					case 1:
						init_timer(3,freqs[4]);
						enable_timer(3);
						max=7851;
							note++;

					break;
					case 2:
						note=0;
					break;
				}
			break;
			case 2:
				switch(note){
					case 0:
						init_timer(3,freqs[1]);
						enable_timer(3);
						max=3306;
							note++;

					break;
					case 1:
						init_timer(3,freqs[2]);
						enable_timer(3);
						max=3711;
							note++;

					break;
					case 2:
						init_timer(3,freqs[3]);
						enable_timer(3);
						max=3925;
							note++;

					break;
					case 3:
						init_timer(3,freqs[4]);
						enable_timer(3);
						max=4410;
							note++;

					break;
					case 4:
						init_timer(3,freqs[5]);
						enable_timer(3);
						max=4948;
							note++;

					break;
					case 5:
						init_timer(3,freqs[6]);
						enable_timer(3);
						max=5555;
							note++;

					break;
					case 6:
						init_timer(3,freqs[7]);
						enable_timer(3);
						max=6091;		
					note++;

					break;
					case 7:
						note=0;
					break;
				}
				break;
			case 3:
				switch(note){
				case 0:
						init_timer(3,freqs[3]);
						enable_timer(3);
						max=15702;//7851
						note++;

					break;
					case 1:
						init_timer(3,freqs[1]);
						enable_timer(3);
						max=13026;	//6613	
					note++;

					break;
					case 2:
						init_timer(3,freqs[0]);
						enable_timer(3);
						max=11792;//5896
							note++;

					break;
					case 3:
						note=0;
					break;
				}
				break;
			default:
				break;
		}
		tempo=0;
		ticks=0;
	}
	
  LPC_TIM3->IR = 1;			/* clear interrupt flag */
}
/******************************************************************************
**                            End Of File
******************************************************************************/
