
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

screw_diam = 3.1;
screw_head = 6; // Screw head. Computer screw has <7mm
insert_diam = 3.6; // Insert diameter 
screw_pylon_diam = 8.5; // Outside diameter

include <YAPPgenerator_v3.scad>

printBaseShell      = false;
printLidShell       = true;

wallThickness = 3.2;
ridgeSlack = 0.2; // Was 0.4, but that's too much.

printerLayerHeight = 0.2;

pcbLength     = 70;
pcbWidth      = 140;
pcbThickness  =  1.5;

lidWallHeight  = 17; // lid+base - pcbthickness - standoffheight?
baseWallHeight = 15;

roundRadius = 1.5;

// SSRs have 22mm height from bottom of PCB (as mounted).
// (20+15) - 7 - 1 <- They should fit.

paddingFront        = 10;
paddingBack         = 10;
paddingRight        = 10;
paddingLeft         = 10;

standoffHeight = 7;
standoffDiameter = 8;
standoffPinDiameter = 3.6;  
standoffHoleSlack = 0.0;

//  Parameters:
//   Required:
//    p(0) = name
//    p(1) = length
//    p(2) = width
//    p(3) = posx
//    p(4) = posy
//    p(5) = Thickness
//    p(6) = standoffHeight 
//    p(7) = standoffDiameter
//    p(8) = standoffPinDiameter
//    p(9) = standoffHoleSlack (default to 0.4)
//   Optional:

//The following can be used to get PCB values elsewhere in the script - not in pcb definition. 
//If "PCB Name" is omitted then "Main" is used
//  pcbLength           --> pcbLength("PCB Name")
//  pcbWidth            --> pcbWidth("PCB Name")
//  pcbThickness        --> pcbThickness("PCB Name") 
//  standoffHeight      --> standoffHeight("PCB Name") 
//  standoffDiameter    --> standoffDiameter("PCB Name") 
//  standoffPinDiameter --> standoffPinDiameter("PCB Name") 
//  standoffHoleSlack   --> standoffHoleSlack("PCB Name") 

pcb =
[
  ["Main",              
  pcbLength, pcbWidth,    
  0,0,    
  pcbThickness,  standoffHeight, standoffDiameter, standoffPinDiameter, standoffHoleSlack],
];

//  Parameters:
//   Required:
//    p(0) = posx
//    p(1) = posy
//   Optional:
//    p(2) = Height to bottom of PCB : Default = standoff_Height
//    p(3) = PCB Gap : Default = -1 : Default for yappCoordPCB=pcb_Thickness, yappCoordBox=0
//    p(4) = standoff_Diameter    Default = standoff_Diameter;
//    p(5) = standoff_PinDiameter Default = standoff_PinDiameter;
//    p(6) = standoff_HoleSlack   Default = standoff_HoleSlack;
//    p(7) = filletRadius (0 = auto size)
//    n(a) = { <yappBoth> | yappLidOnly | yappBaseOnly }
//    n(b) = { <yappPin>, yappHole } // Baseplate support treatment
//    n(c) = { yappAllCorners, yappFrontLeft | <yappBackLeft> | yappFrontRight | yappBackRight }
//    n(d) = { <yappCoordPCB> | yappCoordBox | yappCoordBoxInside }
//    n(e) = { yappNoFillet }
//    n(f) = [yappPCBName, "XXX"] : {Specify a PCB defaults to "Main"
//-------------------------------------------------------------------

pcbStands = 
[
    //[4, 3, 5, -1, 7, yappBoth, yappAllCorners]  //-- Add one pcbStand 5mm from the [0,0,0] corners of the PCB
    
    // Two left
    [5.15, 5.15, yappBaseOnly, yappHole],  //-- Add one pcbStand 5mm from the [0,0,0] corners of the PCB
    [pcbLength - 5.15, 5.15, yappBaseOnly, yappHole],
    
    // Two right
    [5.15, pcbWidth - 5.15, yappBaseOnly, yappHole],
    [pcbLength - 5.15, pcbWidth - 5.2, yappBaseOnly, yappHole],
];

//===================================================================
//  *** Connectors ***
//  Standoffs with hole through base and socket in lid for screw type connections.
//-------------------------------------------------------------------
//  Default origin = yappCoordBox: box[0,0,0]
//  
//  Parameters:
//   Required:
//    p(0) = posx
//    p(1) = posy
//    p(2) = pcbStandHeight
//    p(3) = screwDiameter
//    p(4) = screwHeadDiameter (don't forget to add extra for the fillet)
//    p(5) = insertDiameter
//    p(6) = outsideDiameter
//   Optional:
//    p(7) = PCB Gap : Default = -1 : Default for yappCoordPCB=pcbThickness, yappCoordBox=0
//    p(8) = filletRadius : Default = 0/Auto(0 = auto size)
//    n(a) = { <yappAllCorners>, yappFrontLeft | yappFrontRight | yappBackLeft | yappBackRight }
//    n(b) = { <yappCoordBox> | yappCoordPCB |  yappCoordBoxInside }
//    n(c) = { yappNoFillet }
//    n(d) = [yappPCBName, "XXX"] : {XXX = the PCB name: Default "Main"}

connectors = [
    // Left side (Back, front)
    [5.5, 5.5, 
     10, // Height related to PCB
     screw_diam, // Screw 
     screw_head, // Screw head. Computer screw has <7mm
     insert_diam, // Insert diameter 
     screw_pylon_diam, // Outside diameter
     yappAllCorners,
     yappThroughLid, 
     yappCoordBoxInside],
     
    //[pcbLength, -10, 0, 3, 4, 3, 5],
];


//===================================================================
//  *** Cutouts ***
//    There are 6 cutouts one for each surface:
//      cutoutsBase (Bottom), cutoutsLid (Top), cutoutsFront, cutoutsBack, cutoutsLeft, cutoutsRight
//-------------------------------------------------------------------
//  Default origin = yappCoordBox: box[0,0,0]  
//
//                        Required                Not Used        Note
//----------------------+-----------------------+---------------+------------------------------------
//  yappRectangle       | width, length         | radius        |
//  yappCircle          | radius                | width, length |
//  yappRoundedRect     | width, length, radius |               |     
//  yappCircleWithFlats | width, radius         | length        | length=distance between flats
//  yappCircleWithKey   | width, length, radius |               | width = key width length=key depth
//  yappPolygon         | width, length         | radius        | yappPolygonDef object must be
//                      |                       |               | provided
//----------------------+-----------------------+---------------+------------------------------------
//
//  Parameters:     
//   Required:      
//    p(0) = from Back
//    p(1) = from Left
//    p(2) = width
//    p(3) = length
//    p(4) = radius 
//    p(5) = shape : { yappRectangle | yappCircle | yappPolygon | yappRoundedRect 
//                     | yappCircleWithFlats | yappCircleWithKey }
//  Optional:
//    p(6) = depth : Default = 0/Auto : 0 = Auto (plane thickness)
//    p(7) = angle : Default = 0
//    n(a) = { yappPolygonDef } : Required if shape = yappPolygon specified -
//    n(b) = { yappMaskDef } : If a yappMaskDef object is added it will be used as a mask 
//                             for the cutout.
//    n(c) = { [yappMaskDef, hOffset, vOffset, rotation] } : If a list for a mask is added 
//                              it will be used as a mask for the cutout. With the Rotation 
//                              and offsets applied. This can be used to fine tune the mask
//                              placement within the opening.
//    n(d) = { <yappCoordPCB> | yappCoordBox | yappCoordBoxInside }
//    n(e) = { <yappOrigin>, yappCenter }
//    n(f) = { <yappGlobalOrigin>, yappLeftOrigin } // Only affects Top(lid), Back and Right Faces
//    n(g) = [yappPCBName, "XXX"] : {Specify a PCB defaults to "Main"

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
   [30, 20,
    shellHeight+20, shellWidth - 20*2 , 
    5,
    yappRoundedRect, undef, undef, maskHoneycomb, yappCoordBox]
];

cutoutsBase =
[
  // [9,  ((pcbWidth/2)+0.5), 0, 0, 6.5, yappCircle, yappCenter]   // lens
  // [20 + ventdistance * 0, 32, 3, 70, 1, yappRoundedRect, yappCenter],
  // [20 + ventdistance * 1, 32, 3, 50, 1, yappRoundedRect, yappCenter],
  // [20 + ventdistance * 2, 32, 3, 50, 1, yappRoundedRect, yappCenter],
  // [20 + ventdistance * 3, 32, 3, 70, 1, yappRoundedRect, yappCenter],
];


echo(ventdistance);
echo(cutoutsLid);

// pos, pos, width, height, rect|circle, yappCenter

cutoutsFront = [
   [10, -5, pcbWidth - 10 - 10, 5 + 15 + 2, 1, yappRoundedRect], // RJ45s
];

cutoutsBack = [
   [10, -1, 30, 10, 1, yappRoundedRect], // Power Input
   
   [pcbWidth - 10 - 60, -1, 60, 10, 1, yappRoundedRect], // Auxilliary power
];

cutoutsLeft = [
];

cutoutsRight = [
   [30, -1, 20, 10, 1, yappRoundedRect], // CAN out / in
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

labelsPlane =  
[
    [ 15, 20, 90, 0.4, 
      yappLid,  "FiraCode Nerd Font Mono:style=bold", 7, "smartenough" ],
    [ 25, 20, 90, 0.4, 
      yappLid,  "FiraCode Nerd Font Mono:style=bold", 5, "power HUB v1.0 2025.11" ],
  
];

module hookBaseInside() {
    //translate([65, pcbWidth + paddingLeft + 3]) 
    //cube([36, 2, 20]);
}

YAPPgenerate();