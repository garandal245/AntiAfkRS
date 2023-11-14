# AntiAfkRS

Automatically moves mouse in zone with coordinates provided in set time  has options of pressing right click then escape to cause a mouse interaction and then escape to close and possibly opened context menu 


My first real rust program 

Might be usefull to someone but probably isnt.

compared to my python version i think this was made better but i dont like how the mouse movement works using smoothmove from autopilot crate

only issues that i could find is that it doesnt detect keyboard as of yet which im planning to figure out, and if you dont click into the program manually escape doesnt actually close the context menu

crashes when used on non primary display due to autopilot not supporting 2nd display
