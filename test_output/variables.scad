// cube width
width = 10;
// cube height
height = 20;
union(){
    cube([width,width / 2,height],center = true);
    translate([0,0,height / 2]){
        cylinder(width / 2, r = 5);
    }
}
