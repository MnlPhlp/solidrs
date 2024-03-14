// cube height
HEIGHT = 20;
// cube width
WIDTH = 10;
CYLINDER_HEIGHT = WIDTH / 2;
union(){
    cube([WIDTH,WIDTH / 2,HEIGHT],center = true);
    translate([0,0,HEIGHT / 2]){
        cylinder(CYLINDER_HEIGHT, r = 5);
    }
}
