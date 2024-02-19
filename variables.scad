// cube height
height = 20;
// cube width
width = 10;
// cube depth
depth = 5;
union(){
    cube([width,depth,height],center = true);
    translate([0,0,10]){
        cylinder(10, r = 5);
    }
}
