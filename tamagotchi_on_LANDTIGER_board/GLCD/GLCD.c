/****************************************Copyright (c)**************************************************                         
**
**                                 http://www.powermcu.com
**
**--------------File Info-------------------------------------------------------------------------------
** File name:			GLCD.c
** Descriptions:		Has been tested SSD1289、ILI9320、R61505U、SSD1298、ST7781、SPFD5408B、ILI9325、ILI9328、
**						HX8346A、HX8347A
**------------------------------------------------------------------------------------------------------
** Created by:			AVRman
** Created date:		2012-3-10
** Version:					1.3
** Descriptions:		The original version
**
**------------------------------------------------------------------------------------------------------
** Modified by:			Paolo Bernardi
** Modified date:		03/01/2020
** Version:					2.0
** Descriptions:		simple arrangement for screen usage
********************************************************************************************************/

/* Includes ------------------------------------------------------------------*/
#include "GLCD.h" 
#include "AsciiLib.h"

/* Private variables ---------------------------------------------------------*/
static uint8_t LCD_Code;

/* Private define ------------------------------------------------------------*/
#define  ILI9320    0  /* 0x9320 */
#define  ILI9325    1  /* 0x9325 */
#define  ILI9328    2  /* 0x9328 */
#define  ILI9331    3  /* 0x9331 */
#define  SSD1298    4  /* 0x8999 */
#define  SSD1289    5  /* 0x8989 */
#define  ST7781     6  /* 0x7783 */
#define  LGDP4531   7  /* 0x4531 */
#define  SPFD5408B  8  /* 0x5408 */
#define  R61505U    9  /* 0x1505 0x0505 */
#define  HX8346A		10 /* 0x0046 */  
#define  HX8347D    11 /* 0x0047 */
#define  HX8347A    12 /* 0x0047 */	
#define  LGDP4535   13 /* 0x4535 */  
#define  SSD2119    14 /* 3.5 LCD 0x9919 */

/*******************************************************************************
* Function Name  : Lcd_Configuration
* Description    : Configures LCD Control lines
* Input          : None
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
static void LCD_Configuration(void)
{
	/* Configure the LCD Control pins */
	
	/* EN = P0.19 , LE = P0.20 , DIR = P0.21 , CS = P0.22 , RS = P0.23 , RS = P0.23 */
	/* RS = P0.23 , WR = P0.24 , RD = P0.25 , DB[0.7] = P2.0...P2.7 , DB[8.15]= P2.0...P2.7 */  
	LPC_GPIO0->FIODIR   |= 0x03f80000;
	LPC_GPIO0->FIOSET    = 0x03f80000;
}

/*******************************************************************************
* Function Name  : LCD_Send
* Description    : LCD写数据
* Input          : - byte: byte to be sent
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
static __attribute__((always_inline)) void LCD_Send (uint16_t byte) 
{
	LPC_GPIO2->FIODIR |= 0xFF;          /* P2.0...P2.7 Output */
	LCD_DIR(1)		   				    				/* Interface A->B */
	LCD_EN(0)	                        	/* Enable 2A->2B */
	LPC_GPIO2->FIOPIN =  byte;          /* Write D0..D7 */
	LCD_LE(1)                         
	LCD_LE(0)														/* latch D0..D7	*/
	LPC_GPIO2->FIOPIN =  byte >> 8;     /* Write D8..D15 */
}

/*******************************************************************************
* Function Name  : wait_delay
* Description    : Delay Time
* Input          : - nCount: Delay Time
* Output         : None
* Return         : None
* Return         : None
* Attention		 : None 
*******************************************************************************/
static void wait_delay(int count)
{
	while(count--);
}

/*******************************************************************************
* Function Name  : LCD_Read
* Description    : LCD读数据
* Input          : - byte: byte to be read
* Output         : None
* Return         : 返回读取到的数据
* Attention		 : None
*******************************************************************************/
static __attribute__((always_inline)) uint16_t LCD_Read (void) 
{
	uint16_t value;
	
	LPC_GPIO2->FIODIR &= ~(0xFF);              /* P2.0...P2.7 Input */
	LCD_DIR(0);		   				           				 /* Interface B->A */
	LCD_EN(0);	                               /* Enable 2B->2A */
	wait_delay(30);							   						 /* delay some times */
	value = LPC_GPIO2->FIOPIN0;                /* Read D8..D15 */
	LCD_EN(1);	                               /* Enable 1B->1A */
	wait_delay(30);							   						 /* delay some times */
	value = (value << 8) | LPC_GPIO2->FIOPIN0; /* Read D0..D7 */
	LCD_DIR(1);
	return  value;
}

/*******************************************************************************
* Function Name  : LCD_WriteIndex
* Description    : LCD写寄存器地址
* Input          : - index: 寄存器地址
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
static __attribute__((always_inline)) void LCD_WriteIndex(uint16_t index)
{
	LCD_CS(0);
	LCD_RS(0);
	LCD_RD(1);
	LCD_Send( index ); 
	wait_delay(22);	
	LCD_WR(0);  
	wait_delay(1);
	LCD_WR(1);
	LCD_CS(1);
}

/*******************************************************************************
* Function Name  : LCD_WriteData
* Description    : LCD写寄存器数据
* Input          : - index: 寄存器数据
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
static __attribute__((always_inline)) void LCD_WriteData(uint16_t data)
{				
	LCD_CS(0);
	LCD_RS(1);   
	LCD_Send( data );
	LCD_WR(0);     
	wait_delay(1);
	LCD_WR(1);
	LCD_CS(1);
}

/*******************************************************************************
* Function Name  : LCD_ReadData
* Description    : 读取控制器数据
* Input          : None
* Output         : None
* Return         : 返回读取到的数据
* Attention		 : None
*******************************************************************************/
static __attribute__((always_inline)) uint16_t LCD_ReadData(void)
{ 
	uint16_t value;
	
	LCD_CS(0);
	LCD_RS(1);
	LCD_WR(1);
	LCD_RD(0);
	value = LCD_Read();
	
	LCD_RD(1);
	LCD_CS(1);
	
	return value;
}

/*******************************************************************************
* Function Name  : LCD_WriteReg
* Description    : Writes to the selected LCD register.
* Input          : - LCD_Reg: address of the selected register.
*                  - LCD_RegValue: value to write to the selected register.
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
static __attribute__((always_inline)) void LCD_WriteReg(uint16_t LCD_Reg,uint16_t LCD_RegValue)
{ 
	/* Write 16-bit Index, then Write Reg */  
	LCD_WriteIndex(LCD_Reg);         
	/* Write 16-bit Reg */
	LCD_WriteData(LCD_RegValue);  
}

/*******************************************************************************
* Function Name  : LCD_WriteReg
* Description    : Reads the selected LCD Register.
* Input          : None
* Output         : None
* Return         : LCD Register Value.
* Attention		 : None
*******************************************************************************/
static __attribute__((always_inline)) uint16_t LCD_ReadReg(uint16_t LCD_Reg)
{
	uint16_t LCD_RAM;
	
	/* Write 16-bit Index (then Read Reg) */
	LCD_WriteIndex(LCD_Reg);
	/* Read 16-bit Reg */
	LCD_RAM = LCD_ReadData();      	
	return LCD_RAM;
}

/*******************************************************************************
* Function Name  : LCD_SetCursor
* Description    : Sets the cursor position.
* Input          : - Xpos: specifies the X position.
*                  - Ypos: specifies the Y position. 
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
static void LCD_SetCursor(uint16_t Xpos,uint16_t Ypos)
{
    #if  ( DISP_ORIENTATION == 90 ) || ( DISP_ORIENTATION == 270 )
	
 	uint16_t temp = Xpos;

			 Xpos = Ypos;
			 Ypos = ( MAX_X - 1 ) - temp;  

	#elif  ( DISP_ORIENTATION == 0 ) || ( DISP_ORIENTATION == 180 )
		
	#endif

  switch( LCD_Code )
  {
     default:		 /* 0x9320 0x9325 0x9328 0x9331 0x5408 0x1505 0x0505 0x7783 0x4531 0x4535 */
          LCD_WriteReg(0x0020, Xpos );     
          LCD_WriteReg(0x0021, Ypos );     
	      break; 

     case SSD1298: 	 /* 0x8999 */
     case SSD1289:   /* 0x8989 */
	      LCD_WriteReg(0x004e, Xpos );      
          LCD_WriteReg(0x004f, Ypos );          
	      break;  

     case HX8346A: 	 /* 0x0046 */
     case HX8347A: 	 /* 0x0047 */
     case HX8347D: 	 /* 0x0047 */
	      LCD_WriteReg(0x02, Xpos>>8 );                                                  
	      LCD_WriteReg(0x03, Xpos );  

	      LCD_WriteReg(0x06, Ypos>>8 );                           
	      LCD_WriteReg(0x07, Ypos );    
	
	      break;     
     case SSD2119:	 /* 3.5 LCD 0x9919 */
	      break; 
  }
}

/*******************************************************************************
* Function Name  : LCD_Delay
* Description    : Delay Time
* Input          : - nCount: Delay Time
* Output         : None
* Return         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
static void delay_ms(uint16_t ms)    
{ 
	uint16_t i,j; 
	for( i = 0; i < ms; i++ )
	{ 
		for( j = 0; j < 1141; j++ );
	}
} 


/*******************************************************************************
* Function Name  : LCD_Initializtion
* Description    : Initialize TFT Controller.
* Input          : None
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
void LCD_Initialization(void)
{
	uint16_t DeviceCode;
	
	LCD_Configuration();
	delay_ms(100);
	DeviceCode = LCD_ReadReg(0x0000);		/* 读取屏ID	*/	
	
	if( DeviceCode == 0x9325 || DeviceCode == 0x9328 )	
	{
		LCD_Code = ILI9325;
		LCD_WriteReg(0x00e7,0x0010);      
		LCD_WriteReg(0x0000,0x0001);  	/* start internal osc */
		LCD_WriteReg(0x0001,0x0100);     
		LCD_WriteReg(0x0002,0x0700); 	/* power on sequence */
		LCD_WriteReg(0x0003,(1<<12)|(1<<5)|(1<<4)|(0<<3) ); 	/* importance */
		LCD_WriteReg(0x0004,0x0000);                                   
		LCD_WriteReg(0x0008,0x0207);	           
		LCD_WriteReg(0x0009,0x0000);         
		LCD_WriteReg(0x000a,0x0000); 	/* display setting */        
		LCD_WriteReg(0x000c,0x0001);	/* display setting */        
		LCD_WriteReg(0x000d,0x0000); 			        
		LCD_WriteReg(0x000f,0x0000);
		/* Power On sequence */
		LCD_WriteReg(0x0010,0x0000);   
		LCD_WriteReg(0x0011,0x0007);
		LCD_WriteReg(0x0012,0x0000);                                                                 
		LCD_WriteReg(0x0013,0x0000);                 
		delay_ms(50);  /* delay 50 ms */		
		LCD_WriteReg(0x0010,0x1590);   
		LCD_WriteReg(0x0011,0x0227);
		delay_ms(50);  /* delay 50 ms */		
		LCD_WriteReg(0x0012,0x009c);                  
		delay_ms(50);  /* delay 50 ms */		
		LCD_WriteReg(0x0013,0x1900);   
		LCD_WriteReg(0x0029,0x0023);
		LCD_WriteReg(0x002b,0x000e);
		delay_ms(50);  /* delay 50 ms */		
		LCD_WriteReg(0x0020,0x0000);                                                            
		LCD_WriteReg(0x0021,0x0000);           
		delay_ms(50);  /* delay 50 ms */		
		LCD_WriteReg(0x0030,0x0007); 
		LCD_WriteReg(0x0031,0x0707);   
		LCD_WriteReg(0x0032,0x0006);
		LCD_WriteReg(0x0035,0x0704);
		LCD_WriteReg(0x0036,0x1f04); 
		LCD_WriteReg(0x0037,0x0004);
		LCD_WriteReg(0x0038,0x0000);        
		LCD_WriteReg(0x0039,0x0706);     
		LCD_WriteReg(0x003c,0x0701);
		LCD_WriteReg(0x003d,0x000f);
		delay_ms(50);  /* delay 50 ms */		
		LCD_WriteReg(0x0050,0x0000);        
		LCD_WriteReg(0x0051,0x00ef);   
		LCD_WriteReg(0x0052,0x0000);     
		LCD_WriteReg(0x0053,0x013f);
		LCD_WriteReg(0x0060,0xa700);        
		LCD_WriteReg(0x0061,0x0001); 
		LCD_WriteReg(0x006a,0x0000);
		LCD_WriteReg(0x0080,0x0000);
		LCD_WriteReg(0x0081,0x0000);
		LCD_WriteReg(0x0082,0x0000);
		LCD_WriteReg(0x0083,0x0000);
		LCD_WriteReg(0x0084,0x0000);
		LCD_WriteReg(0x0085,0x0000);
		  
		LCD_WriteReg(0x0090,0x0010);     
		LCD_WriteReg(0x0092,0x0000);  
		LCD_WriteReg(0x0093,0x0003);
		LCD_WriteReg(0x0095,0x0110);
		LCD_WriteReg(0x0097,0x0000);        
		LCD_WriteReg(0x0098,0x0000);  
		/* display on sequence */    
		LCD_WriteReg(0x0007,0x0133);
		
		LCD_WriteReg(0x0020,0x0000);  /* 行首址0 */                                                          
		LCD_WriteReg(0x0021,0x0000);  /* 列首址0 */     
	}

    delay_ms(50);   /* delay 50 ms */	
}

/*******************************************************************************
* Function Name  : LCD_Clear
* Description    : 将屏幕填充成指定的颜色，如清屏，则填充 0xffff
* Input          : - Color: Screen Color
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
void LCD_Clear(uint16_t Color)
{
	uint32_t index;
	
	if( LCD_Code == HX8347D || LCD_Code == HX8347A )
	{
		LCD_WriteReg(0x02,0x00);                                                  
		LCD_WriteReg(0x03,0x00);  
		                
		LCD_WriteReg(0x04,0x00);                           
		LCD_WriteReg(0x05,0xEF);  
		                 
		LCD_WriteReg(0x06,0x00);                           
		LCD_WriteReg(0x07,0x00);    
		               
		LCD_WriteReg(0x08,0x01);                           
		LCD_WriteReg(0x09,0x3F);     
	}
	else
	{	
		LCD_SetCursor(0,0); 
	}	

	LCD_WriteIndex(0x0022);
	for( index = 0; index < MAX_X * MAX_Y; index++ )
	{
		LCD_WriteData(Color);
	}
}

/******************************************************************************
* Function Name  : LCD_BGR2RGB
* Description    : RRRRRGGGGGGBBBBB 改为 BBBBBGGGGGGRRRRR 格式
* Input          : - color: BRG 颜色值  
* Output         : None
* Return         : RGB 颜色值
* Attention		 : 内部函数调用
*******************************************************************************/
static uint16_t LCD_BGR2RGB(uint16_t color)
{
	uint16_t  r, g, b, rgb;
	
	b = ( color>>0 )  & 0x1f;
	g = ( color>>5 )  & 0x3f;
	r = ( color>>11 ) & 0x1f;
	
	rgb =  (b<<11) + (g<<5) + (r<<0);
	
	return( rgb );
}

/******************************************************************************
* Function Name  : LCD_GetPoint
* Description    : 获取指定座标的颜色值
* Input          : - Xpos: Row Coordinate
*                  - Xpos: Line Coordinate 
* Output         : None
* Return         : Screen Color
* Attention		 : None
*******************************************************************************/
uint16_t LCD_GetPoint(uint16_t Xpos,uint16_t Ypos)
{
	uint16_t dummy;
	
	LCD_SetCursor(Xpos,Ypos);
	LCD_WriteIndex(0x0022);  
	
	switch( LCD_Code )
	{
		case ST7781:
		case LGDP4531:
		case LGDP4535:
		case SSD1289:
		case SSD1298:
             dummy = LCD_ReadData();   /* Empty read */
             dummy = LCD_ReadData(); 	
 		     return  dummy;	      
	    case HX8347A:
	    case HX8347D:
             {
		        uint8_t red,green,blue;
				
				dummy = LCD_ReadData();   /* Empty read */

		        red = LCD_ReadData() >> 3; 
                green = LCD_ReadData() >> 2; 
                blue = LCD_ReadData() >> 3; 
                dummy = (uint16_t) ( ( red<<11 ) | ( green << 5 ) | blue ); 
		     }	
	         return  dummy;

        default:	/* 0x9320 0x9325 0x9328 0x9331 0x5408 0x1505 0x0505 0x9919 */
             dummy = LCD_ReadData();   /* Empty read */
             dummy = LCD_ReadData(); 	
 		     return  LCD_BGR2RGB( dummy );
	}
}

/******************************************************************************
* Function Name  : LCD_SetPoint
* Description    : 在指定座标画点
* Input          : - Xpos: Row Coordinate
*                  - Ypos: Line Coordinate 
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
void LCD_SetPoint(uint16_t Xpos,uint16_t Ypos,uint16_t point)
{
	if( Xpos >= MAX_X || Ypos >= MAX_Y )
	{
		return;
	}
	LCD_SetCursor(Xpos,Ypos);
	LCD_WriteReg(0x0022,point);
}

/******************************************************************************
* Function Name  : LCD_DrawLine
* Description    : Bresenham's line algorithm
* Input          : - x1: A点行座标
*                  - y1: A点列座标 
*				   - x2: B点行座标
*				   - y2: B点列座标 
*				   - color: 线颜色
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/	 
void LCD_DrawLine( uint16_t x0, uint16_t y0, uint16_t x1, uint16_t y1 , uint16_t color )
{
	short dx,dy;  
	short temp;       

	if( x0 > x1 )     
	{
		temp = x1;
		x1 = x0;
		x0 = temp;   
	}
	if( y0 > y1 )     
	{
		temp = y1;
		y1 = y0;
		y0 = temp;   
	}
	if(y1>320)
		y1=320;
	if(x1>240)
		x1=240;
  
	dx = x1-x0;       
	dy = y1-y0;       

	if( dx == 0 )     
	{
		do
		{ 
				LCD_SetPoint(x0, y0, color);   
				y0++;
		}
		while( y1 > y0 ); 
	return; 
	}
	if( dy == 0 )      
	{
			do
			{
					LCD_SetPoint(x0, y0, color);   
					x0++;
			}
			while( x1 > x0 ); 
	return;
	}
	
	if( dx > dy )                         
	{
		temp = 2 * dy - dx;                        
			while( x0 != x1 )
			{
				LCD_SetPoint(x0,y0,color);     
				x0++;                         
				if( temp > 0 )                
				{
						y0++;                      
						temp += 2 * dy - 2 * dx; 
			}
					else         
					{
				temp += 2 * dy;             
		}       
			}
			LCD_SetPoint(x0,y0,color);
	}  
	else
	{
		temp = 2 * dx - dy;                             
			while( y0 != y1 )
			{
			LCD_SetPoint(x0,y0,color);     
					y0++;                 
					if( temp > 0 )           
					{
							x0++;               
							temp+=2*dx-2*dy; 
					}
					else
		{
							temp += 2 * dx;
		}
			} 
			LCD_SetPoint(x0,y0,color);
	}
	
} 

/******************************************************************************
* Function Name  : PutChar
* Description    : 将Lcd屏上任意位置显示一个字符
* Input          : - Xpos: 水平坐标 
*                  - Ypos: 垂直坐标  
*				   - ASCI: 显示的字符
*				   - charColor: 字符颜色   
*				   - bkColor: 背景颜色 
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
void PutChar( uint16_t Xpos, uint16_t Ypos, uint8_t ASCI, uint16_t charColor, uint16_t bkColor )
{
	uint16_t i, j;
    uint8_t buffer[16], tmp_char;
    GetASCIICode(buffer,ASCI);  /* 取字模数据 */
    for( i=0; i<16; i++ )
    {
        tmp_char = buffer[i];
        for( j=0; j<8; j++ )
        {
            if( ((tmp_char >> (7 - j)) & 0x01) == 0x01 )
            {
                LCD_SetPoint( Xpos + j, Ypos + i, charColor );  /* 字符颜色 */
            }
            else
            {
                LCD_SetPoint( Xpos + j, Ypos + i, bkColor );  /* 背景颜色 */
            }
        }
    }
}

/******************************************************************************
* Function Name  : GUI_Text
* Description    : 在指定座标显示字符串
* Input          : - Xpos: 行座标
*                  - Ypos: 列座标 
*				   - str: 字符串
*				   - charColor: 字符颜色   
*				   - bkColor: 背景颜色 
* Output         : None
* Return         : None
* Attention		 : None
*******************************************************************************/
void GUI_Text(uint16_t Xpos, uint16_t Ypos, uint8_t *str,uint16_t Color, uint16_t bkColor, uint8_t type)
{
		uint8_t x,y;
	
	
    uint8_t TempChar;
    do
    {
        TempChar = *str++; 
				if(type==0)
				{
					PutChar( Xpos, Ypos, TempChar, Color, bkColor ); 
					x=8;
					y=16;
				}else if(type==1)
				{
					my_PutChar( Xpos, Ypos, TempChar, Color, bkColor ); 
					x=16;
					y=32;
				}else if(type==2)
				{
					my_PutChar2( Xpos, Ypos, TempChar, Color, bkColor ); 
					x=32;
					y=64;
				}
        if( Xpos < MAX_X - x )
        {
            Xpos += x;
        } 
        else if ( Ypos < MAX_Y - y )
        {
            Xpos = 0;
            Ypos += y;
        }   
        else
        {
            Xpos = 0;
            Ypos = 0;
        }    
    }
    while ( *str != 0 );
}

void LCD_DrawRect(uint16_t x0,uint16_t y0, uint16_t w,uint16_t h,uint16_t color)
{
	int x1,i;
	
	x1=x0+w;
	
	for(i=0;i<h;i++)
	{
		LCD_DrawLine(x0,y0,x1,y0,color);
		y0++;
	}
}

void my_PutChar( uint16_t Xpos, uint16_t Ypos, uint8_t ASCI, uint16_t charColor, uint16_t bkColor )
{
	uint16_t i, j;
    uint8_t buffer[16], tmp_char;
    GetASCIICode(buffer,ASCI);  
    for( i=0; i<16; i++ )
    {
        tmp_char = buffer[i];
        for( j=0; j<8; j++ )
        {
            if( ((tmp_char >> (7 - j)) & 0x01) == 0x01 )
            {
                LCD_SetPoint( Xpos + (j*2), Ypos + (i*2), charColor );  
								LCD_SetPoint( Xpos + (j*2)+1, Ypos + (i*2), charColor );
            }
            else
            {
                LCD_SetPoint( Xpos + (j*2), Ypos + (i*2), bkColor ); 
								LCD_SetPoint( Xpos + (j*2)+1, Ypos + (i*2), bkColor );
            }
						if( ((tmp_char >> (7 - j)) & 0x01) == 0x01 )
            {
                LCD_SetPoint( Xpos + (j*2), Ypos + (i*2)+1, charColor );  
								LCD_SetPoint( Xpos + (j*2)+1, Ypos + (i*2)+1, charColor );
            }
            else
            {
                LCD_SetPoint( Xpos + (j*2), Ypos + (i*2)+1, bkColor ); 
								LCD_SetPoint( Xpos + (j*2)+1, Ypos + (i*2)+1, bkColor );
            }
        }
    }
}

void my_PutChar2( uint16_t Xpos, uint16_t Ypos, uint8_t ASCI, uint16_t charColor, uint16_t bkColor )
{
	uint16_t i, j,k,l;
    uint8_t buffer[16], tmp_char;
    GetASCIICode(buffer,ASCI);  
    for( i=0; i<16; i++ )
    {
        tmp_char = buffer[i];
        for( j=0; j<8; j++ )
        {
					for(k=0;k<4;k++)
					{
						if(((tmp_char>>(7-j))&0x01)==0x01)
						{
							for(l=0;l<4;l++)
							{
								LCD_SetPoint( Xpos + (j*4)+l, Ypos + (i*4)+k, charColor );
							}
						}else
						{
							for(l=0;l<4;l++)
							{
								LCD_SetPoint( Xpos + (j*4)+l, Ypos + (i*4)+k, bkColor );
							}
						}
					}
        }
    }
}

void Draw_pacman(uint16_t x0, uint16_t y0,uint16_t color,int8_t type)//0=normale,1=eat dx,2=eat sx,3=nulla,4=2 occhi,5=nulla
{
	int8_t circle[]={-8,-5,-4,-3,-3,-2,-2,-2,-2,-2,-2,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,0,-1,0,-1,0,-1,0,-1,0,-1,0,-1,0,0,-1,0,0,-1,0,0,0,-1,0,0,0,0,-1,0,0,0,0,0,0,0};
	int8_t ellipse[]={-1,-2,-2,-1,-1,-1,0,0,0};
	int x1,i;
	
	y0=y0-62;
	x1=x0;
	if(type==0||type==2||type==4)
	{
		for(i=0;i<60;i++)
		{
			x0=x0+circle[i];
			y0++;
			x1=x1-circle[i];
			LCD_DrawLine(x0,y0,x1,y0,color);
		}
		for(i=59;i>0;i--)
		{
			x0=x0-circle[i];
			y0++;
			x1=x1+circle[i];
			LCD_DrawLine(x0,y0,x1,y0,color);
		}
	}else
	{
		if(type==1)
		{
			for(i=0;i<60;i++)
			{
				x0=x0+circle[i];
				y0++;
				if(i<=10)
				{
					x1=x1-circle[i];
				}else
				{
					if(i%2==0)
					{
						x1=x1-0;
					}else
					{
						x1=x1-1;
					}
				}
				LCD_DrawLine(x0,y0,x1,y0,color);
			}
			for(i=59;i>0;i--)
			{
				x0=x0-circle[i];
				y0++;
				if(i<=10)
				{
					x1=x1+circle[i];
				}else
				{
					if(i%2==0)
					{
						x1=x1+0;
					}else
					{
						x1=x1+1;
					}
				}
				LCD_DrawLine(x0,y0,x1,y0,color);
			}
		}else
		{
			if(type==3)
			{
				for(i=0;i<60;i++)
				{
					x1=x1-circle[i];
					y0++;
					if(i<=10)
					{
						x0=x0+circle[i];
					}else
					{
						if(i%2==0)
						{
							x0=x0+0;
						}else
						{
							x0=x0+1;
						}
					}
					LCD_DrawLine(x0,y0,x1,y0,color);
				}
				for(i=59;i>0;i--)
				{
					x1=x1+circle[i];
					y0++;
					if(i<=10)
					{
						x0=x0-circle[i];
					}else
					{
						if(i%2==0)
						{
							x0=x0-0;
						}else
						{
							x0=x0-1;
						}
					}
					LCD_DrawLine(x0,y0,x1,y0,color);
				}
			}else
			{
			}
		}
	}
	
	if(type!=4)
	{
		if(type==3||type==2)
		{
			x0=x0-3;
		}else
		{
			x0=x0+15;
		}
		y0=y0-98;
		x1=x0+2;
		for(i=0;i<9;i++)
		{
			x0=x0+ellipse[i];
			y0++;
			x1=x1-ellipse[i];
			LCD_DrawLine(x0,y0,x1,y0,Black);
		}
		for(i=8;i>0;i--)
		{
			x0=x0-ellipse[i];
			y0++;
			x1=x1+ellipse[i];
			LCD_DrawLine(x0,y0,x1,y0,Black);
		}
		if(color!=Black)
		{
			if(type==2||type==3)
			{
				LCD_DrawRect(x0-5,y0-10,4,4,White);
			}else
			{
				LCD_DrawRect(x0+6,y0-10,4,4,White);
			}
		}
	}else
	{
		if(type==4)
		{
			y0=y0-98;
			x0=x0-15;
			x1=x0+2;
			for(i=0;i<9;i++)
			{
				x0=x0+ellipse[i];
				y0++;
				x1=x1-ellipse[i];
				LCD_DrawLine(x0,y0,x1,y0,Black);
			}
			for(i=8;i>0;i--)
			{
				x0=x0-ellipse[i];
				y0++;
				x1=x1+ellipse[i];
				LCD_DrawLine(x0,y0,x1,y0,Black);
			}
			x0=x0+45;
			y0=y0-17;
			x1=x0+2;
			for(i=0;i<9;i++)
			{
				x0=x0+ellipse[i];
				y0++;
				x1=x1-ellipse[i];
				LCD_DrawLine(x0,y0,x1,y0,Black);
			}
			for(i=8;i>0;i--)
			{
				x0=x0-ellipse[i];
				y0++;
				x1=x1+ellipse[i];
				LCD_DrawLine(x0,y0,x1,y0,Black);
			}
			LCD_DrawRect(101,129,4,4,White);
			LCD_DrawRect(134,129,4,4,White);
		}
	}
	
	
}
void mounth(int a)
{
	int8_t right[]={1,0,1,0,0,1,0,0,1,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0};
	int8_t left[]={-1,-2,-2,-2,-2,-2,-2,-3,-2,-2,-2,-2,-2,-2,-3,-2,-2,-2,-2,-2,-3,-2,-2,-2,-2};
	int x0,x1,y0,i;
	
	x0=120+54;
	y0=160-27;
	x1=x0;
	
	for(i=0;i<25;i++)
	{
		x0=x0+left[i];
		x1=x1+right[i];
		y0++;
		if(a==0)
		{
			LCD_SetPoint(x0,y0,Black);
			LCD_DrawLine(x0+1,y0,x1+1,y0,Black);
		}else
		{
			LCD_DrawLine(x0,y0,x1,y0,Yellow);
			LCD_SetPoint(x1,y0,Black);
		}
	}
	for(i=24;i>0;i--)
	{
		x0=x0-left[i];
		x1=x1-right[i];
		y0++;
		if(a==0)
		{
			LCD_SetPoint(x0,y0,Black);
			LCD_DrawLine(x0+1,y0,x1+1,y0,Black);
		}else
		{
			LCD_DrawLine(x0,y0,x1,y0,Yellow);
			LCD_SetPoint(x1,y0,Black);
		}
	}
}

void LCD_start(uint16_t color, uint16_t bkColor)
{
	uint8_t up[]={0,1,1,1,1,1,0};
	int xs,ys,i,x0,y0,y1;
	LCD_Clear(bkColor);

	xs=56;
	ys=10;
	LCD_DrawRect(5,11,7,11,Blue);
	x0=11;
	y0=10;
	y1=22;
	for(i=0;i<7;i++)
	{
		x0++;
		y0=y0-up[i];
		y1=y1+up[i];
		LCD_DrawLine(x0,y0,x0,y1,Blue);
	}
	
	GUI_Text(xs,ys,(uint8_t*) " Age: ",color,bkColor,0);
	for(xs=xs+48;xs<168;xs=xs+8)
	{
		if(xs==120||xs==144)
		{
			GUI_Text(xs,ys,(uint8_t*) ":",color,bkColor,0);
		}else
		{
			GUI_Text(xs,ys,(uint8_t*) "0",color,bkColor,0);
		}
	}
	GUI_Text(xs,ys,(uint8_t*) " ",color,bkColor,0);
	
	GUI_Text(16,30,(uint8_t*) " Happiness ",color,bkColor,0);
	GUI_Text(144,30,(uint8_t*) " Satiety ",color,bkColor,0);
	
	LCD_DrawRect(44,50,29,16,Green);
	LCD_DrawRect(74,55,2,6,Green);
	LCD_DrawRect(46,52,25,12,bkColor);
	LCD_DrawRect(47,53,5,10,Green);
	LCD_DrawRect(53,53,5,10,Green);
	LCD_DrawRect(59,53,5,10,Green);	
	LCD_DrawRect(65,53,5,10,Green);
	
	LCD_DrawRect(164,50,29,16,Green);
	LCD_DrawRect(194,55,2,6,Green);
	LCD_DrawRect(166,52,25,12,bkColor);
	LCD_DrawRect(167,53,5,10,Green);
	LCD_DrawRect(173,53,5,10,Green);
	LCD_DrawRect(179,53,5,10,Green);	
	LCD_DrawRect(185,53,5,10,Green);
	
	//bordo pulsanti
	LCD_DrawRect(0,249,240,3,color);
	LCD_DrawRect(0,249,3,91,color);
	LCD_DrawRect(119,249,3,91,color);
	LCD_DrawRect(237,249,3,91,color);
	LCD_DrawRect(0,317,240,3,color);
	
	GUI_Text(28,270,(uint8_t*) "Meal",color,bkColor,1);
	GUI_Text(140,270,(uint8_t*) "Snack",color,bkColor,1);
	
	Draw_pacman(120,160,Yellow,0);
}

void Draw_phantom(uint16_t x0, uint16_t y0)
{
	int8_t top[]={0,-2,-2,-1,-1,-1,-1,0,-1,0,-1,0,0,0,0,0};
	int8_t bottom[]={0,-1,-1,-1,1,1,1,-1,-1,-1,1,1,1,-1,-1,-1};
	int y1,i;
	
	y1=y0+18;
	for(i=0;i<16;i++)
	{
		y0=y0+top[i];
		x0++;
		y1=y1+bottom[i];
		LCD_DrawLine(x0,y0,x0,y1,Blue);
	}
	for(i=15;i>0;i--)
	{
		y0=y0-top[i];
		x0++;
		y1=y1-bottom[i];
		LCD_DrawLine(x0,y0,x0,y1,Blue);
	}
}

void Draw_ball(uint16_t x0, uint16_t y0)
{
	int8_t right[]={0,-3,-2,-1,-1,-1,-1,0,-1,0,0,-1,0,0,0};
	int x1,i;
		
	x0=x0-3;
	x1=x0+6;
	y0=y0-15;
	
	for(i=0;i<15;i++)
	{
		x0=x0+right[i];
		y0++;
		x1=x1-right[i];
		LCD_DrawLine(x0,y0,x1,y0,White);
	}
	for(i=15;i>0;i--)
	{
		x0=x0-right[i];
		y0++;
		x1=x1+right[i];
		LCD_DrawLine(x0,y0,x1,y0,White);
	}
}

void Draw_heart(uint16_t x0, uint16_t y0, uint16_t Color)
{
	int8_t up[]={7,1,1,0,1,0,-1,0,-1,-1,-2,2,1,1,0,1,0,-1,0,-1,-1};
	//int8_t down[];
	int y1,i,a=1;
		
	x0=x0-11;
	y0=y0-10;
	y1=y0-1;
	for(i=0;i<21;i++)
	{
		y1=y1+a;
		y0=y0-up[i];
		x0++;
		LCD_DrawLine(x0,y0,x0,y1,Color);
		if(i==10)
			a=-1;
	}		
}

void Draw_volume(uint8_t type)
{	
	LCD_DrawRect(21,5,12,25,Black);
	
	if(type>=1)
	{
		LCD_DrawRect(21,13,2,1,Blue);
		LCD_DrawRect(22,14,2,5,Blue);
		LCD_DrawRect(21,19,2,1,Blue);
	}
	if(type>=2)
	{
		LCD_DrawRect(24,10,2,1,Blue);
		LCD_DrawRect(25,11,2,2,Blue);
		LCD_DrawRect(26,13,2,7,Blue);
		LCD_DrawRect(25,20,2,2,Blue);
		LCD_DrawRect(24,22,2,1,Blue);
	}
	if(type>=3)
	{
		LCD_DrawRect(27,7,2,1,Blue);
		LCD_DrawRect(28,8,2,2,Blue);
		LCD_DrawRect(29,10,2,3,Blue);
		LCD_DrawRect(30,13,2,7,Blue);
		LCD_DrawRect(29,20,2,3,Blue);
		LCD_DrawRect(28,23,2,2,Blue);
		LCD_DrawRect(27,25,2,1,Blue);
	}
}
/*********************************************************************************************************
      END FILE
*********************************************************************************************************/
