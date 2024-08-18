mod common;

#[test]
fn hello_world() {
    let source = br#"
pageextension 70074160 MS_CustomerListExt extends "Customer List"
{
    trigger OnOpenPage() {
        Message('Hello world');
    }
}"#;

    let target = br#"
pageextension 70074160 MS_CustomerListExt extends "Customer List"
{
    trigger OnOpenPage()
    begin
        Message('Hello world');
    end;
}"#;
    common::test_writer_output_is_target(source, target);
}

#[test]
fn if_statements() {
    let source = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage() {
        if a == true and 6 * 5 + 1 == 4 {
            DoSomething();
        } else {
            DoSomethingElse();
        }

        if a == true {
            DoSomething();
        }
    }
}"#;

    let target = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage()
    begin
        if (a = true) and (((6 * 5) + 1) = 4) then begin
            DoSomething();
        end else begin
            DoSomethingElse();
        end;

        if a = true then begin
            DoSomething();
        end;
    end;
}"#;
    common::test_writer_output_is_target(source, target);
}

#[test]
fn return_statements() {
    let source = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage() {
        return a - 1 * 7;
        return;
        return /* test comment */;
    }
}"#;

    let target = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage()
    begin
        exit(a - (1 * 7));
        exit;
        exit /* test comment */;
    end;
}"#;
    common::test_writer_output_is_target(source, target);
}


#[test]
fn switch_statements() {
    let source = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage() {
        switch 7 * 4 / 4 {
        case 4:
            DoOne();
            switch 3 {
            case 3:
                DoInner();
            }
            DoTwo();
        /* comment 1 */ case /* comment2 */ 3 /* comment3 */:
            DoOnlyOne();
        default:
            DoOne();
            DoTwo();
        }
    }
}"#;

    let target = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage()
    begin
        case (7 * 4) / 4 of
            4:
                begin
                    DoOne();
                    case 3 of
                        3:
                            DoInner();
                    end;
                    DoTwo();
                end;
            /* comment 1 */ /* comment2 */ 3 /* comment3 */:
                DoOnlyOne();
            else
                begin
                    DoOne();
                    DoTwo();
                end;
        end;
    end;
}"#;
    common::test_writer_output_is_target(source, target);
}

#[test]
fn vars() {
    let source = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage() {
        MoreContent0();
        var/* test */ a: Integer /* test */;
        MoreContent1();
        var "Quoted Name": Label "test label";
    }
    trigger OnBeforeGetRecord() {
        var IsRecordSaved:/* test1 */ Boolean /* test2 */ = false;
    }
}"#;

    let target = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage()
    var
        /* test */ a: Integer /* test */;
        "Quoted Name": Label "test label";
    begin
        MoreContent0();
        MoreContent1();
    end;
    trigger OnBeforeGetRecord()
    var
        IsRecordSaved:/* test1 */ Boolean;
    begin
        IsRecordSaved /* test2 */ := false;
    end;
}"#;
    common::test_writer_output_is_target(source, target);
}

#[test]
fn operators() {
    let source = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage() {
        a = (5 == 7 % 4 div 2) != true;
    }
}"#;

    let target = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage()
    begin
        a := (5 = ((7 mod 4) div 2)) <> true;
    end;
}"#;
    common::test_writer_output_is_target(source, target);
}

#[test]
fn loops() {
    let source = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage() {
        for a = 1 to 5 {
            a += 1;
        }
        foreach a in b {
            a += 1;
        }
        while true {
            a += 1;
        }
        repeat {
            Hello();
        } until Bye();
    }
}"#;

    let target = br#"
page 70074160 MS_CustomerListExt
{
    trigger OnOpenPage()
    begin
        for a := 1 to 5 do begin
            a += 1;
        end;
        foreach a in b do begin
            a += 1;
        end;
        while true do begin
            a += 1;
        end;
        repeat
            Hello();
        until Bye();
    end;
}"#;
    common::test_writer_output_is_target(source, target);
}

#[test]
fn global_vars() {
    let source = br#"
pageextension 70074160 MS_CustomerListExt extends "Customer List"
{
    var
        a: Boolean;
    protected var
        a: Boolean;
        b: Label 'test';

    trigger OnOpenPage() {
        Message('Hello world');
    }
}"#;

    let target = br#"
pageextension 70074160 MS_CustomerListExt extends "Customer List"
{
    var
        a: Boolean;
    protected var
        a: Boolean;
        b: Label 'test';

    trigger OnOpenPage()
    begin
        Message('Hello world');
    end;
}"#;
    common::test_writer_output_is_target(source, target);
}

#[test]
fn object_with_property() {
    let source = br#"
table 50100 "Main Vendors"
{
    TableRelation = Vendor."No." where ("Balance (LCY)" == filter (!= 10000));
}"#;

    let target = br#"
table 50100 "Main Vendors"
{
    TableRelation = Vendor."No." where ("Balance (LCY)" = filter (<> 10000));
}"#;
    common::test_writer_output_is_target(source, target);
}


