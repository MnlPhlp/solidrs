// cube height
height = 20;
// cube width
width = 10;
union(){
    cube([width,width / 2,height],center = true);
    top = height/2;
    translate([0,0,top]){
        cylinder(10, r = 5);
    }
}
