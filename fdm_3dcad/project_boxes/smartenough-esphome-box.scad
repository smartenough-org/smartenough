
// esphome uni board.

/*
      padding-back|<------pcb length --->|<padding-front
                            RIGHT
        0    X-axis ---> 
        +----------------------------------------+   ---
        |                                        |    ^
        |                                        |   padding-right 
      Y |                                        |    v
      | |    -5,y +----------------------+       |   ---              
 B    a |         | 0,y              x,y |       |     ^              F
 A    x |         |                      |       |     |              R
 C    i |         |                      |       |     | pcb width    O
 K    s |         |                      |       |     |              N
        |         | 0,0              x,0 |       |     v              T
      ^ |    -5,0 +----------------------+       |   ---
      | |                                        |    padding-left
      0 +----------------------------------------+   ---
        0    X-as --->
                          LEFT
*/





// Local variables
ventdistance = 14;


include <YAPPgenerator_v3.scad>

printBaseShell      = true;
printLidShell       = true;

wallThickness = 3.2;
ridgeSlack = 0.4;

printerLayerHeight = 0.2;

pcbLength     = 90;
pcbWidth      = 90;
pcbThickness  =  2;

lidWallHeight  = 15;
baseWallHeight = 20;

paddingFront        = 5;
paddingBack         = 5;
paddingRight        = 15;
paddingLeft         = 15;

standoffHeight = 10;
standoffDiameter = 8;
standoffPinDiameter = 3.6;  
standoffHoleSlack = 0.0;


pcb =
[
  ["Main",              pcbLength, pcbWidth,    0,0,    pcbThickness,  standoffHeight, standoffDiameter, standoffPinDiameter, standoffHoleSlack]
 //,["4x SSR",  57, 55,  0, pcbWidth + 10,  1.6,  3, 7, 2.4]
 //,["Voltage Detect 2",  15.1,71.5+3,  pcbLength + 4 + 15.1, 0,  1.6,  3, 7, 2.4]
];


pcbStands = 
[
    //[4, 3, 5, -1, 7, yappBoth, yappAllCorners]  //-- Add one pcbStand 5mm from the [0,0,0] corners of the PCB
    
    // Two left
    [4, 6.4, yappBaseOnly, yappHole],  //-- Add one pcbStand 5mm from the [0,0,0] corners of the PCB
    [pcbLength - 6, 6.4, yappBaseOnly, yappHole],
    
    // Two right
    [4, pcbWidth - 13.6, yappBaseOnly, yappHole],
    [pcbLength - 6, pcbWidth - 13.6, yappBaseOnly, yappHole],
];

connectors = [
    [-1.73, -11.73, 
     0, // Height related to PCB
     3.1, // Screw 
     7, // Screw head. Computer screw has <7mm
     3.6, // Insert diameter 
     8.5, // Outside diameter
     yappAllCorners, yappThroughLid],
    //[pcbLength, -10, 0, 3, 4, 3, 5],
];

cutoutsLid =
[
   // [9,  ((pcbWidth/2)+0.5), 0, 0, 6.5, yappCircle, yappCenter]   // lens
   // from back, from left, width, length, radius
   /*
   [20 + ventdistance * 0, pcbWidth/2 - 70/2, 3, 70, 1, yappRoundedRect],
   [20 + ventdistance * 1, pcbWidth/2 - 50/2, 3, 50, 1, yappRoundedRect],
   [20 + ventdistance * 2, pcbWidth/2 - 50/2, 3, 50, 1, yappRoundedRect],
   [20 + ventdistance * 3, pcbWidth/2 - 70/2, 3, 70, 1, yappRoundedRect],
   */
];

cutoutsBase =
[
    //[9,  ((pcbWidth/2)+0.5), 0, 0, 6.5, yappCircle, yappCenter]   // lens
  // [20 + ventdistance * 0, 32, 3, 70, 1, yappRoundedRect, yappCenter],
  // [20 + ventdistance * 1, 32, 3, 50, 1, yappRoundedRect, yappCenter],
  // [20 + ventdistance * 2, 32, 3, 50, 1, yappRoundedRect, yappCenter],
  // [20 + ventdistance * 3, 32, 3, 70, 1, yappRoundedRect, yappCenter],
];


echo(ventdistance);
echo(cutoutsLid);

// pos, pos, width, height, rect|circle, yappCenter

cutoutsFront = [
   [12, -3, 7+16+35, 13, 0, yappRectangle], // 1 Wire
   
];

cutoutsLeft = [
   [5, -3, 30, 13, 0, yappRectangle], // ANALOG
   [pcbWidth-13-30, -3, 30, 13, 0, yappRectangle], // INPUT RS/PWR
];

cutoutsRight = [
   [5, -3, 40, 13, 0, yappRectangle], // DIGIT
   [pcbWidth-7-25, -3, 25, 13, 0, yappRectangle], // MOSFET
];

boxMounts = [
    [15/2, 5, 10, 4, yappRight, yappBase],
    [shellLength - 15 - 5, 5, 10, 4, yappRight, yappBase],
    
    [15/2, 5, 10, 4, yappLeft, yappBase],
    [shellLength - 15 - 5, 5, 10, 4, yappLeft, yappBase],
    
    [shellLength/2 - 10, 5, 10, 4, yappBack, yappBase],
];

debug                = false;
showOrientation      = true;      //-> Show the Front/Back/Left/Right labels : only in preview
showPCB              = true;      //-> Show the PCB in red : only in preview 
showSwitches         = true;      //-> Show the switches (for pushbuttons) : only in preview 
showButtonsDepressed = false;     //-> Should the buttons in the Lid On view be in the pressed position
showOriginCoordBox   = false;     //-> Shows red bars representing the origin for yappCoordBox : only in preview 
showOriginCoordBoxInside  = false;  //-> Shows blue bars representing the origin for yappCoordBoxInside : only in preview 
showOriginCoordPCB        = false;  //-> Shows blue bars representing the origin for yappCoordBoxInside : only in preview 
showMarkersPCB            = false;  //-> Shows black bars corners of the PCB : only in preview 
showMarkersCenter         = false;  //-> Shows magenta bars along the centers of all faces  


snapJoins   =   
[
  // [12, 5, yappLeft, yappRight],
  // [(pcbLength+paddingFront*2)/2, 5, yappLeft, yappRight],
  // [(pcbLength+paddingFront*2)-12, 5, yappLeft, yappRight]
];

YAPPgenerate();