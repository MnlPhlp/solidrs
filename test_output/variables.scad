// cube width
width = 10;
// cube height
height = 20;
cylinder_height = width / 2;
union(){
    cube([width,width / 2,height],center = true);
    translate([0,0,height / 2]){
        cylinder(cylinder_height, r = 5);
    }
}
