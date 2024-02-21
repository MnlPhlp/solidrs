// cube depth
depth = 5;
// cube height
height = 20;
// cube width
width = 10;
union(){
    cube([width,depth,height],center = true);
    translate([0,0,height / 2]){
        cylinder(10, r = 5);
    }
}
