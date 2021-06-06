INSTRUCTIONS:

For Help or questions regarding this product, please contact:

David L. Couch, Geodesist
National Imagery and Mapping Agency
3200 South Second Street
St. Louis Mo 63118

This package contains 4 files:
1.  Egm96 (data file containing spherical harmonic coefficients)
2.  CorrDtoE (data file containing correction coefficients)
3.  ClenGui.exe (executable file)
4.  README ( text file with instructions)
5.  Shortcut (drag and drop to the desktop)

Procedure:

1.  Place data files and executable in the same directory.

2.  If desired, drag the shortcut to the desktop.

3.  Double click the shortcut or the executable icon to run the program.

4.  A message window will pop up requesting the user to wait 50 seconds for the coefficient files to load.  This time will vary depending on the platform.  Press OK.  If the files cannot load, a message will indicate this.  

5.  A second message window will pop up when coefficient files are read.  Press OK.  

6.  The graphical user interface (GUI) will appear.  Enter coordinates in degrees minutes and seconds.  Leave the Geodetic Height "0" if the geoid height is desired.  

7.  Press "Run" to compute the geoid height (the height anomaly is computed if geodetic height is not equal to zero).  

8.  Press "Clear All" to clear the input boxes.  

9.  Press "Close" to exit the application.  

****************************** WARNING ************************************
DO NOT USE A NON-ZERO VALUE FOR GEODETIC HEIGHT (ellipsoid height) IF THE GEOID HEIGHT IS DESIRED.  The use of a geodetic height other than zero results in a height anomaly and NOT a geoid height.  
***************************************************************************

Comments:

1.  For southern or western hemispheres, the user need only to apply a negative sign to the degrees.  The negative sign will automatically get applied to the minutes and seconds components.  The user can also apply the negative signs during input.  

2.  This program will not compute a geoid at +/- 90 degrees latitude, rather the latitude is internally changed to +/- 89.9999999... instead.  

3.  The "CorrDtoE" correction coefficient file is not the same as the Corrcoeff file downloadable from the NIMA Web site.  Please use the enclosed CorrDtoE for this application.  The "Egm96" harmonic coefficient files are the same.  