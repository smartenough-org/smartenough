
// Local variables
ventdistance = 14;


include <YAPPgenerator_v3.scad>

pcbLength     = 90;
pcbWidth      = 70;
pcbThickness  =  2;

lidWallHeight  = 20;
baseWallHeight = 15;

paddingFront        = 15;
paddingBack         = 15;
paddingRight        = 15;
paddingLeft         = 15;

standoffPinDiameter = 2.5;
pcbStands = 
[
    [4, 3, 5, -1, 7, yappBoth,
    yappAllCorners]  //-- Add one pcbStand 5mm from the [0,0,0] corners of the PCB
];


cutoutsLid =
[
    //[9,  ((pcbWidth/2)+0.5), 0, 0, 6.5, yappCircle, yappCenter]   // lens
   [20 + ventdistance * 0, 32, 3, 70, 1, yappRoundedRect, yappCenter],
   [20 + ventdistance * 1, 32, 3, 50, 1, yappRoundedRect, yappCenter],
   [20 + ventdistance * 2, 32, 3, 50, 1, yappRoundedRect, yappCenter],
   [20 + ventdistance * 3, 32, 3, 70, 1, yappRoundedRect, yappCenter],
];

cutoutsBase =
[
    //[9,  ((pcbWidth/2)+0.5), 0, 0, 6.5, yappCircle, yappCenter]   // lens
   [20 + ventdistance * 0, 32, 3, 70, 1, yappRoundedRect, yappCenter],
   [20 + ventdistance * 1, 32, 3, 50, 1, yappRoundedRect, yappCenter],
   [20 + ventdistance * 2, 32, 3, 50, 1, yappRoundedRect, yappCenter],
   [20 + ventdistance * 3, 32, 3, 70, 1, yappRoundedRect, yappCenter],
];


echo(ventdistance);
echo(cutoutsLid);

cutoutsFront =
[
    [pcbLength - 40, 10, 40, 15, 0, yappRectangle, yappCenter] // USB
];

cutoutsLeft =
[
    [15, 10, 10, 15, 0, yappRectangle, yappCenter] // USB
];

boxMounts   = 
[
    [15/2, 5, 10, 4, yappRight, yappBase],
    [shellLength - 15 - 5, 5, 10, 4, yappRight, yappBase],
    
    [15/2, 5, 10, 4, yappLeft, yappBase],
    [shellLength - 15 - 5, 5, 10, 4, yappLeft, yappBase],
    
    [shellLength/2 - 10, 5, 10, 4, yappBack, yappBase],
];

debug = false;
showOrientation           = true;       //-> Show the Front/Back/Left/Right labels : only in preview
showPCB                   = true;      //-> Show the PCB in red : only in preview 
showSwitches              = true;      //-> Show the switches (for pushbuttons) : only in preview 
showButtonsDepressed      = false;      //-> Should the buttons in the Lid On view be in the pressed position
showOriginCoordBox        = false;      //-> Shows red bars representing the origin for yappCoordBox : only in preview 
showOriginCoordBoxInside  = false;      //-> Shows blue bars representing the origin for yappCoordBoxInside : only in preview 
showOriginCoordPCB        = false;      //-> Shows blue bars representing the origin for yappCoordBoxInside : only in preview 
showMarkersPCB            = false;      //-> Shows black bars corners of the PCB : only in preview 
showMarkersCenter         = false;      //-> Shows magenta bars along the centers of all faces  


snapJoins   =   
[
   [12, 5, yappLeft, yappRight],
   [(pcbLength+paddingFront*2)/2, 5, yappLeft, yappRight],
   [(pcbLength+paddingFront*2)-12, 5, yappLeft, yappRight]
];

YAPPgenerate();